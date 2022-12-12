use canvasapi::prelude::*;
use clap::{Parser, Subcommand, ValueEnum};

/// A tool to interact with the Canvas API.
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// The URL of the Canvas API (env: CANVAS_BASE_URL).
    #[clap(long)]
    url: Option<String>,
    /// The access token for the Canvas API (env: CANVAS_ACCESS_TOKEN).
    #[clap(long)]
    token: Option<String>,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Get your course information.
    Courses,
    /// Get all the assignments of a course.
    Assignments {
        /// The ID of the course.
        #[clap(value_parser)]
        id: usize,
    },
    /// Get all the information of a course.
    Course {
        /// The ID of the course.
        #[clap(value_parser)]
        id: usize,
    },
    /// Get the users enrolled to a course.
    Users {
        /// The ID of the course.
        #[clap(value_parser)]
        course_id: usize,
        /// The enrolment type of the user.
        #[clap(short, long = "type", value_parser)]
        type_: Option<Enrollment>,
    },
    /// Download files of a course.
    Files {
        /// The ID of the course.
        #[clap(value_parser)]
        id: usize,
        /// The output directory.
        #[clap(long, value_parser)]
        out_dir: Option<std::path::PathBuf>,
    },
    /// Search for a course based on a search string.
    SearchCourse {
        /// The search terms used.
        #[clap(value_parser)]
        search: String,
        /// Only return courses with public content.
        #[clap(long)]
        public_only: bool,
        /// Only return courses that allow self enrollment.
        #[clap(long)]
        open_enrollment_only: bool,
    },
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Clone, ValueEnum)]
enum Enrollment {
    Teacher,
    Student,
    StudentView,
    TA,
    Observer,
    Designer,
}

impl From<Enrollment> for canvasapi::prelude::EnrollmentType {
    fn from(val: Enrollment) -> Self {
        match val {
            Enrollment::Teacher => canvasapi::prelude::EnrollmentType::Teacher,
            Enrollment::Student => canvasapi::prelude::EnrollmentType::Student,
            Enrollment::StudentView => canvasapi::prelude::EnrollmentType::StudentView,
            Enrollment::TA => canvasapi::prelude::EnrollmentType::TA,
            Enrollment::Observer => canvasapi::prelude::EnrollmentType::Observer,
            Enrollment::Designer => canvasapi::prelude::EnrollmentType::Designer,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok().unwrap();

    let args = Args::parse();

    let base_url = if let Some(url) = args.url {
        // TODO: check that the URL is valid.
        url.to_string()
    } else {
        std::env::var("CANVAS_BASE_URL")
            .expect("Pass the Canvas URL as an environment variable, or pass it to the command.")
    };

    let canvas_token = if let Some(token) = args.token {
        token
    } else {
        std::env::var("CANVAS_ACCESS_TOKEN")
            .expect("Pass the Canvas access token using an environment variable, or pass it to the command.")
    };

    let canvas = CanvasInformation::new(&base_url, &canvas_token);

    let data = match args.command {
        Commands::Courses => {
            let courses = Course::courses()
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            serde_json::to_string_pretty(&courses).unwrap()
        }
        Commands::Assignments { id } => {
            let course = Canvas::get_course(id)
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            let assignments = course
                .get_assignments()
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            serde_json::to_string_pretty(&assignments).unwrap()
        }
        Commands::Course { id } => {
            let course = Canvas::get_course(id)
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            serde_json::to_string_pretty(&course).unwrap()
        }
        Commands::Users { course_id, type_ } => {
            let course = Canvas::get_course(course_id)
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            let users = course.get_users().unwrap();

            let users = if let Some(type_) = type_ {
                users.add_parameter(<Enrollment as Into<EnrollmentType>>::into(type_))
            } else {
                users
            };

            let users = match users.fetch(&canvas).await {
                Ok(users) => users.inner(),
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };

            serde_json::to_string_pretty(&users).unwrap()
        }
        Commands::Files { id, out_dir } => {
            let course = Canvas::get_course(id)
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            let files = course
                .get_files()
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();

            if let Some(out_dir) = out_dir.as_ref() {
                if !out_dir.as_path().exists() {
                    std::fs::create_dir_all(out_dir.as_path()).unwrap();
                }
            }

            for file in files {
                let path = if let Some(ref out_dir) = out_dir {
                    out_dir.as_path().to_str().unwrap()
                } else {
                    "."
                };

                file.download(&canvas, path).await.unwrap();
            }
            "".to_string()
        }
        Commands::SearchCourse {
            search,
            public_only,
            open_enrollment_only,
        } => {
            let results = Canvas::search_course(search, public_only, open_enrollment_only)
                .unwrap()
                .fetch(&canvas)
                .await
                .unwrap()
                .inner();
            serde_json::to_string_pretty(&results).unwrap()
        }
    };

    println!("{data}");
}

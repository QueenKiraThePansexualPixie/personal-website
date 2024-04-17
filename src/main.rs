use chrono::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod typing {
    use chrono::LocalResult;

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct KiraDate {
        year: u16,
        month: u8,
        day: u8,
    }

    #[allow(dead_code)]
    impl KiraDate {
        pub(crate) fn new(year: u16, month: u8, day: u8) -> KiraDate {
            KiraDate { year, month, day }
        }

        /// ```
        /// let date = KiraDate::new(2024, 2, 1);
        ///
        /// assert_eq!(date.to_string("Y-M-D"), format!("{}-{}-{}", date.get_year(), date.get_month(), date.get_day()));
        /// assert_eq!(date.to_string("D/M/Y"), format!("{}/{}/{}", date.get_day(), date.get_month(), date.get_year()));
        /// assert_eq!(date.to_string("Y.M.D"), format!("{}.{}.{}", date.get_day(), date.get_month(), date.get_year()));
        /// assert_eq!(date.to_string(""), format!("{}/{}/{}", date.get_day(), date.get_month(), date.get_year()));
        /// ```
        pub(crate) fn to_string(&self, format_string: &str) -> String {
            match format_string.to_lowercase().as_str() {
                "y-m-d" => format!("{}-{}-{}", self.year, self.month, self.day),
                "d-m-y" => format!("{}-{}-{}", self.day, self.month, self.year),
                "y/m/d" => format!("{}/{}/{}", self.year, self.month, self.day),
                "d/m/y" => format!("{}/{}/{}", self.day, self.month, self.year),
                "y.m.d" => format!("{}.{}.{}", self.year, self.month, self.year),
                "d.m.y" => format!("{}.{}.{}", self.day, self.month, self.year),
                _ => format!("{}/{}/{}", self.day, self.month, self.year),
            }
        }

        pub(crate) fn get_day(&self) -> u8 {
            self.day
        }

        pub(crate) fn get_month(&self) -> u8 {
            self.month
        }

        pub(crate) fn get_year(&self) -> u16 {
            self.year
        }

        pub(crate) fn is_leap_year(&self) -> bool {
            self.year % 4 == 0 && self.year % 100 != 0 || self.year % 400 == 0
        }

        pub(crate) fn is_valid(&self) -> bool {
            self.month > 0 && self.month <= 12 && self.day > 0 && self.day <= 31
        }

        pub(crate) fn to_chrono_datetime(&self) -> LocalResult<DateTime<Utc>> {
            Utc.with_ymd_and_hms(
                self.year as i32,
                self.month as u32,
                self.day as u32,
                0,
                0,
                0,
            )
        }
    }
}
use typing::*;

mod content;
use content::*;

mod pages {
    use super::*;

    #[derive(Clone, Routable, PartialEq)]
    pub(crate) enum Route {
        #[at("/")]
        Home,
        #[at("/skills")]
        Skills,
        #[at("/skills/:skill")]
        Skill { skill: String },
        #[at("/achievements")]
        Achievements,
        #[at("/achievements/:achievement")]
        Achievement { achievement: String },
        #[at("/creations")]
        Creations,
        #[at("/creations/:creation")]
        Creation { creation: String },
        #[at("/articles")]
        Articles,
        #[at("/articles/:article")]
        Article { article: String },
        #[at("/contact")]
        Contact,
        #[not_found]
        #[at("/404")]
        NotFound,
    }

    #[function_component(Home)]
    pub(crate) fn home() -> Html {
        html! {
            <div>
                <h1>{"Home"}</h1>

                <p>{
                    "Hi, I'm Kira H, and I somehow exist, unfortunately for you."
                }</p>

                <hr />

                <p>
                    {"I was born in the small town of Wincanton in rural England. "}
                    {"It was a Wednesday, Wednesday the 14th of September, in 2005. "}
                </p>
            </div>
        }
    }

    #[function_component(Skills)]
    pub(crate) fn skills(SkillListProperties { skills }: &SkillListProperties) -> Html {
        html! {
            <div>
                <h1>{"Skills"}</h1>

                <p>{"These are my skills."}</p>

                <hr />

                <div>{skills.get_html()}</div>
            </div>
        }
    }

    #[function_component(Achievements)]
    pub(crate) fn achievements(
        AchievementListProperties { achievements }: &AchievementListProperties,
    ) -> Html {
        html! {
            <div>
                <h1>{"Achievements"}</h1>

                <p>{"These are my achievements."}</p>

                <hr />

                <div>{achievements.get_html()}</div>
            </div>
        }
    }

    #[function_component(Creations)]
    pub(crate) fn creations(CreationListProperties { creations }: &CreationListProperties) -> Html {
        html! {
            <div>
                <h1>{"Creations"}</h1>

                <p>{"These are my creations."}</p>

                <hr />

                <div>{creations.get_html()}</div>
            </div>
        }
    }

    #[function_component(Articles)]
    pub(crate) fn articles(ArticleListProperties { articles }: &ArticleListProperties) -> Html {
        html! {
            <div>
                <h1>{"Articles"}</h1>

                <p>{"These are my articles."}</p>

                <hr />

                <div>{articles.get_html()}</div>
            </div>
        }
    }

    #[function_component(Contact)]
    pub(crate) fn contact() -> Html {
        html! {
            <div>
                <h1>{"Contact"}</h1>

                <p>{
                    "If you have any questions, feel free to contact me."
                }</p>

                <div class={"contact-details-container"}>
                    <a
                        class={"contact-icon"}
                        href={"mailto:kira.hudson.v0@gmail.com"}
                        title={"kira.hudson.v0@gmail.com"}
                        id={"Email"}
                        target={"_blank"}
                    >
                        <i class={"fa-solid fa-square-envelope"}></i>
                    </a>

                    <a
                        class={"contact-icon"}
                        href={"https://github.com/QueenKiraThePansexualPixie/"}
                        id={"GitHub"}
                        title={"@QueenKiraThePansexualPixie"}
                        target={"_blank"}
                    >
                        <i class={"fa-brands fa-square-github"}></i>
                    </a>

                    <a
                        class={"contact-icon"}
                        href={"https://www.tumblr.com/blog/kira-is-pan/"}
                        id={"Tumblr"}
                        title={"@kira-is-pan"}
                        target={"_blank"}
                    >
                        <i class={"fa-brands fa-square-tumblr"}></i>
                    </a>

                    <a
                        class={"contact-icon"}
                        href={"https://www.instagram.com/kirathepanpixie/"}
                        id={"Instagram"}
                        title={"@kirathepanpixie"}
                        target={"_blank"}
                    >
                        <i class={"fa-brands fa-square-instagram"}></i>
                    </a>

                    <a
                        class={"contact-icon"}
                        href={"https://www.pinterest.co.uk/kirathepansexualpixie/"}
                        id={"Pinterest"}
                        title={"@kirathepansexualpixie"}
                        target={"_blank"}
                    >
                        <i class={"fa-brands fa-square-pinterest"}></i>
                    </a>

                    <a
                        class={"contact-icon"}
                        href={"https://www.reddit.com/user/KiraThePanPixie/"}
                        id={"Reddit"}
                        title={"@KiraThePanPixie"}
                        target={"_blank"}
                    >
                        <i class={"fa-brands fa-square-reddit"}></i>
                    </a>
                </div>
            </div>
        }
    }

    #[function_component(NotFound)]
    pub(crate) fn not_found() -> Html {
        html! {
            <div>
                <h1>{"Error 404"}</h1>

                <p>{"Error 404 : Page Not Found"}</p>

                <p>{"Please navigate your way back to the main site"}</p>

                <p>
                    <b>{"EMERGENCY EXIT:"}</b>{" "}<a href={"/"}>{"Home"}</a>
                </p>
            </div>
        }
    }
}
use pages::*;

#[function_component(App)]
fn app() -> Html {
    let skills: SkillList = SkillList::new(vec![
        Skill {
            id: 0,
            name: "Rust".to_string(),
            areas: vec![
                "Backend".to_string(),
                "Frontend".to_string(),
                "Game Development".to_string(),
                "Scripting".to_string(),
                "Systems Programming".to_string(),
                "Web Development".to_string(),
            ],
            competency: "Beginner".to_string(),
            description: "High-level systems programming language, designed for interacting more safely with low-level concepts.".to_string(),
        },
        Skill {
            id: 1,
            name: "C++".to_string(),
            areas: vec![
                "Backend".to_string(),
                "Database Development".to_string(),
                "Digital Graphics".to_string(),
                "Frontend".to_string(),
                "Game Development".to_string(),
                "Scripting".to_string(),
                "Systems Programming".to_string(),
                "Web Development".to_string(),
            ],
            competency: "Novice".to_string(),
            description: "Low-level, high-control, systems programming language. Higher level than roughly 2 of the hundreds of programming language that exist in today's landscape.".to_string(),
        },
    ]);

    let achievements: AchievementList = AchievementList::new(vec![Achievement {
        id: 0,
        name: "<NAME>".to_string(),
        completed: KiraDate::new(2023, 5, 24),
        areas: vec!["Programming".to_string(), "Web Development".to_string()],
        tools: vec!["Visual Studio Code".to_string(), "VS Code".to_string()],
        skills: vec![],
        description: "".to_string(),
    }]);

    let creations: CreationList = CreationList::new(vec![Creation {
        id: 0,
        name: "<NAME>".to_string(),
        completed: KiraDate::new(2023, 9, 16),
        areas: vec![],
        tools: vec![],
        skills: vec![],
        description: "".to_string(),
    }]);

    let articles: ArticleList = ArticleList::new(vec![Article {
        id: 0,
        title: "<NAME>".to_string(),
        published: KiraDate::new(2023, 8, 17),
        topics: vec![],
        summary: "".to_string(),
        content: html! {
            <div>
                <h1>{"<NAME>"}</h1>
            </div>
        },
    }]);

    html! {
        <div>
            <header  id={"header"}>
                <h1>{ "Hello, world!" }</h1>
                <img src={"icon.png"} alt={"Website Icon"} />
            </header>
            <ul id={"nav"}>
                <li>
                    <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Skills}>{ "Skills" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Achievements}>{ "Achievements" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Creations}>{ "Creations" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Articles}>{ "Articles" }</Link<Route>>
                </li>
                <li>
                    <Link<Route> to={Route::Contact}>{ "Contact" }</Link<Route>>
                </li>
            </ul>
            <main id={"main"}>
                <BrowserRouter>
                    <Switch<Route>
                        render={move |route: Route| {
                            match route {
                                Route::Home => html! { <Home /> },
                                Route::Skills => html! { <Skills skills={skills.clone()} /> },
                                Route::Skill { skill } => html! {
                                    <ContentComponent content={
                                        match skills.find_first_with_name(&skill) {
                                            Some(skill) => Content::Skill(skill.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Achievements => html! { <Achievements achievements={achievements.clone()} /> },
                                Route::Achievement { achievement } => html! {
                                    <ContentComponent content={
                                        match achievements.find_first_with_name(&achievement) {
                                            Some(achievement) => Content::Achievement(achievement.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Creations => html! { <Creations creations={creations.clone()} /> },
                                Route::Creation { creation } => html! {
                                    <ContentComponent content={
                                        match creations.find_first_with_name(&creation) {
                                            Some(creation) => Content::Creation(creation.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Articles => html! { <Articles articles={articles.clone()} /> },
                                Route::Article { article } => html! {
                                    <ContentComponent content={
                                        match articles.find_first_with_title(&article) {
                                            Some(article) => Content::Article(article.to_owned()),
                                            None => Content::NotFound,
                                        }
                                    } />
                                },
                                Route::Contact => html! { <Contact /> },
                                Route::NotFound => html! { <NotFound /> },
                            }
                        }}
                    />
                </BrowserRouter>
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

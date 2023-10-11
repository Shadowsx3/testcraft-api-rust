use crate::models::requests::accessibility_request::CheckAccessibilityRequest;
use crate::models::requests::automate_tests_request::{AutomateTestsIdeasRequest, AutomateTestsRequest};
use crate::models::requests::generate_ideas_request::GenerateIdeasRequest;
use crate::utils::html::is_valid_html;
use crate::utils::openai::call_openai_api;
use actix_web::{post, web, HttpResponse, Responder, Scope};
use serde_json::json;

const ERROR_INVALID_ELEMENT: &str = "Invalid html element.";

#[post("/generate-ideas")]
async fn generate_ideas(data: web::Json<GenerateIdeasRequest>) -> impl Responder {
    if let Some(ping) = data.ping {
        if ping {
            return HttpResponse::Ok().json(json!({"pong": true}));
        }
    }

    let source = data.source_code.clone().unwrap_or_default();

    if !is_valid_html(&source) {
        return HttpResponse::BadRequest().json(json!({"error": ERROR_INVALID_ELEMENT}));
    }

    let role = "You are a Software Test Consultant";

    let prompt = format!(
        "Generate test ideas based on the html element below. \
        Focus on tests that are user-oriented and not referring to html elements such as divs or classes. \
        Include negative tests. If possible, add some creative test scenarios. \
        Format the output as unordered lists, with a heading for each required list. \
        Html: \n```\n{}```\n\n\
        Format the output as the following example:\n\
        Positive Tests:\n\
        <Idea 1>\n\n\
        Negative Tests:\n\
        <Idea 1>\n\n\
        Creative Test Scenarios:\n\
        <Idea 1>",
        &source
    );

    call_openai_api(prompt, role).await
}

#[post("/automate-tests")]
async fn automate_tests(data: web::Json<AutomateTestsRequest>) -> impl Responder {
    if !is_valid_html(&data.source_code) {
        return HttpResponse::BadRequest().json(json!({"error": ERROR_INVALID_ELEMENT}));
    }

    let role = "You are a Test Automation expert";

    let mut prompt = format!(
        "Generate {} tests using {} based on the html element below. \
        Use {} as the baseUrl. Generate as many tests as possible. \
        Always add assertions. \
        Do not include explanatory or introductory text. The output must be all {} code.",
        data.framework, data.language, data.base_url, data.language,
    );

    if data.framework == "playwright" {
        prompt += " Use playwright/test library.";
    }

    if let Some(pom) = data.pom {
        if pom {
            prompt += " Create page object models and use them in the tests. \
            Selectors must be encapsulated in properties. \
            Actions must be encapsulated in methods. \
            Include a comment to indicate where each file starts.";
        }
    }

    prompt += &format!("Html: \n```\n{}\n```\n", &data.source_code);

    call_openai_api(prompt, role).await
}

#[post("/check-accessibility")]
async fn check_accessibility(data: web::Json<CheckAccessibilityRequest>) -> impl Responder {
    if !is_valid_html(&data.source_code) {
        return HttpResponse::BadRequest().json(json!({"error": ERROR_INVALID_ELEMENT}));
    }

    let role = "You are an expert on Web Accessibility";

    let prompt = format!(
        "Check the HTML element below for accessibility issues according to WCAG 2.1. \
        Think about this step by step. First, assess the element against each criterion. Then, report the result in the specified format. \
        For the criteria that cannot be assessed just by looking at the HTML, create accessibility tests. \
        In the report, each criterion must be a link to the reference documentation.\n\n\
        Html:\n```{}```\n\n\
        Format the output as the following example:\n\
        - Issues\n\
        - Conformance Level A -\n\
        - Issue:\n\
        - Criteria:\n\
        - Solution:\n\n\
        - Conformance Level AA -\n\
        - Issue:\n\
        - Criteria:\n\
        - Solution:\n\n\
        - Conformance Level AAA -\n\
        - Issue:\n\
        - Criteria:\n\
        - Solution:\n\n\
        - Suggested Tests\n\
        - Test:\n\
        - Criteria:\n\
        - Test Details:",
        data.source_code
    );

    return call_openai_api(prompt, role).await;
}

#[post("/automate-tests-ideas")]
async fn automate_tests_ideas(data: web::Json<AutomateTestsIdeasRequest>) -> impl Responder {
    if !is_valid_html(&data.source_code) {
        return HttpResponse::BadRequest().json(json!({"error": ERROR_INVALID_ELEMENT}));
    }

    let role = "You are a Test Automation expert";
    let line_tab = "\n\t";

    let ideas = &data.ideas;
    let ideas_list = ideas.join(&line_tab);

    let mut prompt = format!(
        "Using the following html:\n\n\
        Html:\n```\n{}\n```\n\n\
        Generate {} tests using {} for the following Test Cases:\n\n\
        TestCases:\n```\n{}\n```\n\n\
        Use {} as the baseUrl.\n\
        Always add assertions.\n\
        Do not include explanatory or introductory text. The output must be all {} code.",
        &data.source_code, data.framework, data.language, ideas_list, data.base_url, data.language
    );

    if data.framework == "playwright" {
        prompt += " Use playwright/test library.";
    }

    if let Some(pom) = data.pom {
        if pom {
            prompt += " Create page object models and use them in the tests. \
            Selectors must be encapsulated in properties. \
            Actions must be encapsulated in methods. \
            Include a comment to indicate where each file starts.";
        }
    }

    return call_openai_api(prompt, role).await;
}

pub fn get_scope() -> Scope {
    web::scope("")
        .service(generate_ideas)
        .service(automate_tests)
        .service(check_accessibility)
        .service(automate_tests_ideas)
}

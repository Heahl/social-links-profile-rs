use crate::components::counter_btn::Button;
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

<div class="container">
        <div class="header-container">
            <img
                src="/assets/images/avatar-jessica.jpeg"
                alt="avatar-jessica"
                height="200"
                width="400"
            />

            <h1>Jessica Randall</h1>
            <h2>London, United Kingdom</h2>
        </div>

        <div class="buttons-container">
            <p>"\"Front-end developer and avid reader\""</p>
            <Button title="GitHub"/>
            <Button title="Frontend Mentor" />
            <Button title="LinkedIn" />
            <Button title="Twitter" />
            <Button title="Instagram" />
        </div>
</div>
        </ErrorBoundary>
    }
}

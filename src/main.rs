use yew::prelude::*;
use yew_router::prelude::*;

// ===================================================================================
// for {username}.github.io/{repo_name}
// replace 'yew-template-for-github.io' to your repo name
/*
#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
    #[at("/yew-test/")]
    Home,
    #[at("/yew-test/:s")]
    Route,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/yew-test/about")]
    About,
    #[not_found]
    #[at("/yew-test/404")]
    NotFound,
}

fn root_route(routes: &RootRoute) -> Html {
    match routes {
        RootRoute::Home => html! { <p class="text-4xl">{ "Yew Template. Texto agregado " }</p> },
        RootRoute::Route => html! {
            <Switch<Route> render={Switch::render(switch)} />
        },
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::About => html! { <p>{ "About" }</p> },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}
*/

// ===================================================================================
// for {username}.github.io

#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
      #[at("/")]
      Home,
      #[at("/about")]
      About,
      #[not_found]
      #[at("/404")]
      NotFound,
  }

  fn root_route(routes: &RootRoute) -> Html {
      match routes {
          RootRoute::Home => html! { <p class="text-4xl">{ "Yew Test username.github.io real " }</p> },
          RootRoute::About => html! { <p>{ "About" }</p> },
          RootRoute::NotFound => html! { <p>{ "Not Found" }</p> },
      }
  }

// ===================================================================================

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        // ********************************************************
        // **    basename is not supported on yew 0.19.0 yet.    **
        // <BrowserRouter basename="/yew-template-for-github-io/">
        //     <Switch<Route> render={Switch::render(switch)} />
        // </BrowserRouter>
        // ********************************************************
        <BrowserRouter>
            <Switch<RootRoute> render={Switch::render(root_route)} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::start_app::<App>();
}

use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Router, Routes, Route, RouteProps, RouterProps, RoutesProps};


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,

        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <Cats/> }/>
                </Routes>
            </main>
        </Router>
    }
}

async fn fetch_cats(_: u32) -> Option<Vec<String>> { 
    Some(vec!["first".to_string(), "second".to_string()]) 
}

#[component]
fn Cats(cx: Scope) -> impl IntoView {
    let (cat_count, _) = create_signal::<u32>(cx, 1);
    let cats = create_resource(cx, cat_count, |count | fetch_cats(count));
    log!("Have created resource");
    view! { cx,
        <div>
          <Suspense fallback=move || view! { cx, <p>"Loading (Suspense Fallback)..."</p> }>
            {move || {
                log!("Entering cat code");
                let results = cats.read(cx).map(|data| match data {
                  None => view! { cx,  <pre>"Error"</pre> }.into_any(),
                  Some(cats) => view! { cx,
                    <div>{
                      cats.iter()
                        .map(|text| {
                          view! { cx,
                            <p>{text}</p>
                          }
                        })
                        .collect::<Vec<_>>()
                    }</div>
                  }.into_any(),
                });
                match results {
                    None => log!("None retured"),
                    Some(_) => log!("Some returned"),
                }
                // let results = match results {
                //     None => Some(view! { cx,
                //         <div>
                //         <p>"Strange"</p>
                //         <p>"Odd"</p>
                //         </div>
                //     }.into_any()),
                //     Some(inner) => Some(inner)
                // };
                log!("Leaving cat code");
                results
              }
            }
          </Suspense>
        </div>
      }
      
}


use leptos::*;


#[component]
fn App() -> impl IntoView {
    view! {
        <h1>"Iteration"</h1>
        <h2>"Static List"</h2>
        <p>"Use this pattern if the list itself is static."</p>
        <StaticList    length=8   />
        <h2>"Dynamic List"</h2>
        <p>"Use this pattern if the rows in your list will change."</p>
        <DynamicList    initial_length=7  />
    }
}


#[component]
fn StaticList( length: usize ) -> impl IntoView {
    
    let counters = (1..=length).map(|idx| create_signal(idx));
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        }) .collect::<Vec<_>>();

    view! {
     <ul>{counter_buttons}</ul>
    }
}


#[component]
fn DynamicList( initial_length: usize  ) -> impl IntoView {
    
    let mut next_counter_id = initial_length;

    let initial_counters = (0..initial_length).map(|id| (id, create_signal(id + 1))).collect::<Vec<_>>();

    let (counters, set_counters) = create_signal(initial_counters);

    let add_counter = move |_| {
        
        let sig = create_signal(next_counter_id + 1);
    
        set_counters.update(move |counters| {counters.push((next_counter_id, sig)) });
        
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add Counter"
            </button>
            <ul>
                
                <For each=counters  key=|counter| counter.0
                    children=move |(id, (count, set_count))| {
                        view! {
                            <li>
                                <button on:click=move |_| set_count.update(|n| *n += 1) >
                                    {count}
                                </button>
                                <button on:click=move |_| {
                                        set_counters.update(|counters| { counters.retain(|(counter_id, (signal, _))| {
                                            
                                                if counter_id == &id {
                                                    signal.dispose();
                                                }
                                                counter_id != &id
                                            })
                                        });
                                    }
                                >  "Remove" </button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App)
}

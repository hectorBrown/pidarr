use leptos::mount::mount_to_body;
use leptos::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let radarr_addr = RwSignal::new("127.0.0.1:7878".to_string());
    let qbit_addr = RwSignal::new("127.0.0.1:8888".to_string());

    view! {
        <table>
            <tr>
                <th><p> Radarr address: </p></th>
                <th><p>
                    <input type="text"
                        bind:value=radarr_addr
                    />
                </p></th>
            </tr>
            <tr>
                <th><p> qBittorrent address: </p></th>
                <th><p>
                    <input type="text"
                        bind:value=qbit_addr
                    />
                </p></th>
            </tr>
        </table>
        <button on:click=|_| {
                //TODO: save addresses
            }
        >Save</button>

    }
}

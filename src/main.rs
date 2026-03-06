#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Viewport tag for mobile responsiveness
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }

        // SEO, Security, and Favicon
        // SEO, Security, and Local Favicon
        document::Link { rel: "icon", href: asset!("/assets/logo.png") }
        document::Meta { name: "description", content: "NomadLibre: Libre software that moves with you. Building stable, cross-platform utilities in Rust." }
        document::Meta { name: "keywords", content: "Rust, Open Source, Dioxus, Linux, Fedora, DevOps, Libre Software" }
        document::Meta { property: "og:title", content: "NomadLibre Studio" }
        document::Meta { property: "og:description", content: "Libre software that moves with you, across every device you own." }
        document::Meta { property: "og:image", content: asset!("/assets/logo.png") }
        document::Meta { property: "og:url", content: "https://nomadlibre.studio" }
        document::Meta { property: "og:type", content: "website" }
        document::Meta {
            "http-equiv": "Content-Security-Policy",
            content: "default-src 'self'; img-src 'self' data: https://github.com; style-src 'self' 'unsafe-inline'; script-src 'self' 'wasm-unsafe-eval';"
        }
        
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        // Main Background: Gruvbox Dark (bg0), Text: Gruvbox Light (fg)
        div { class: "min-h-screen bg-[#282828] text-[#ebdbb2] font-mono p-4 md:p-8",

            // Top Navigation Bar
            nav { class: "max-w-6xl mx-auto border-4 border-[#1d2021] bg-[#3c3836] p-4 mb-8 shadow-[6px_6px_0px_0px_#1d2021] flex flex-wrap justify-between items-center relative z-50 gap-4",

                // Logo and Name
                div { class: "flex items-center gap-4",
                    img { src: asset!("/assets/logo.png"), alt: "NomadLibre Logo", class: "w-10 h-10 border-2 border-[#1d2021] bg-white" }
                    div { class: "font-bold text-2xl tracking-tighter text-[#fabd2f]", "NomadLibre" }
                }

                // Navigation Links
                div { class: "flex flex-wrap gap-6 items-center",

                    // Projects Dropdown Menu
                    div { class: "relative group",
                        button { class: "hover:text-[#fabd2f] font-bold cursor-pointer uppercase", "Projects ▾" }

                        // Dropdown Content
                        div { class: "absolute left-0 mt-4 w-56 border-4 border-[#1d2021] bg-[#282828] shadow-[6px_6px_0px_0px_#1d2021] hidden group-hover:block",
                            ul { class: "flex flex-col text-sm",
                                li { a { href: "#", class: "block px-4 py-3 hover:bg-[#3c3836] hover:text-[#fabd2f] border-b-4 border-[#1d2021] font-bold", "MoveLivre" } }
                                li { a { href: "#", class: "block px-4 py-3 hover:bg-[#3c3836] hover:text-[#fabd2f] border-b-4 border-[#1d2021] font-bold", "Personal Finance" } }
                                li { a { href: "#", class: "block px-4 py-3 hover:bg-[#3c3836] hover:text-[#fabd2f] border-b-4 border-[#1d2021] font-bold", "Board Games" } }
                                li { a { href: "#", class: "block px-4 py-3 hover:bg-[#3c3836] hover:text-[#fabd2f] border-b-4 border-[#1d2021] font-bold", "Mobile Terminal" } }
                                li { a { href: "#", class: "block px-4 py-3 hover:bg-[#3c3836] hover:text-[#fabd2f] border-b-4 border-[#1d2021] font-bold", "Secure P2P Chat" } }
                                li { a { href: "#", class: "block px-4 py-3 hover:bg-[#3c3836] hover:text-[#fabd2f] font-bold", "The Meme App" } }
                            }
                        }
                    }

                    a { href: "#", class: "hover:text-[#fabd2f] font-bold uppercase", "Tutorials" }
                    a { href: "#", class: "hover:text-[#fabd2f] font-bold uppercase", "Docs" }
                    a { href: "https://github.com/NomadLibre", target: "_blank", class: "hover:text-[#fabd2f] font-bold uppercase underline decoration-2 underline-offset-4", "GitHub" }
                }
            }

            main { class: "max-w-6xl mx-auto",

                // Hero Section - Gruvbox Yellow
                header { class: "border-4 border-[#1d2021] bg-[#fabd2f] text-[#282828] p-8 md:p-12 mb-8 shadow-[8px_8px_0px_0px_#1d2021]",
                    h1 { class: "text-4xl md:text-5xl font-bold mb-6 uppercase tracking-tight", "Pretty boring opensource project is in the making." }

                    // The Tagline
                    div { class: "inline-block border-4 border-[#1d2021] bg-[#282828] text-[#ebdbb2] p-3 mb-6 shadow-[4px_4px_0px_0px_#1d2021]",
                        p { class: "text-xl md:text-2xl font-bold", "Libre software that moves with you, across every device you own." }
                    }

                    p { class: "text-xl font-bold", "Building stable, cross-platform utilities in Rust. Expect terminal apps, privacy-first tools, and board games." }
                }

                // Grid Content
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 mb-8",

                    // Block 1 - Gruvbox Blue
                    div { class: "border-4 border-[#1d2021] bg-[#83a598] text-[#282828] p-8 shadow-[8px_8px_0px_0px_#1d2021] md:col-span-2",
                        h2 { class: "text-3xl font-bold mb-2 uppercase", "Why Rust?" }
                        p { class: "font-bold", "I needed a stack that compiles to everything: Win, Mac, Linux (RPM/DEB/Flatpak), FreeBSD, iOS, Android, and WebAssembly. Memory safety is just a bonus." }
                    }

                    // Block 2 - Neutral Beige (Solo Dev / Hiring)
                    div { class: "border-4 border-[#1d2021] bg-[#ebdbb2] text-[#282828] p-8 shadow-[8px_8px_0px_0px_#1d2021]",
                        h2 { class: "text-2xl font-bold mb-2 uppercase", "Team Size: 1" }
                        p { class: "font-bold", "I am a solo developer currently in a bit of a money crunch. I absolutely do not expect free work (not even for an intern!), so it is just me building this out right now. If you want to support, starring the repos helps a lot!" }
                    }

                    // Block 3 - Gruvbox Green (Tutorials)
                    div { class: "border-4 border-[#1d2021] bg-[#b8bb26] text-[#282828] p-8 shadow-[8px_8px_0px_0px_#1d2021]",
                        h2 { class: "text-2xl font-bold mb-2 uppercase", "Tutorials & Docs" }
                        p { class: "font-bold", "I will add video tutorials for consumers and comprehensive documentation right here once the products are actually ready." }
                    }

                    // Block 4 - Gruvbox Purple (Philosophy)
                    div { class: "border-4 border-[#1d2021] bg-[#d3869b] text-[#282828] p-8 shadow-[8px_8px_0px_0px_#1d2021] md:col-span-2",
                        h2 { class: "text-3xl font-bold mb-2 uppercase", "My Philosophy" }
                        p { class: "font-bold", "Offline first, KISS philosophy. No data harvesting. No telemetry. Just solid tools built for longevity and stability using official distribution channels." }
                    }
                }

                // Broken Website Acknowledgment Banner - Gruvbox Orange
                footer { class: "border-4 border-[#1d2021] bg-[#fe8019] text-[#282828] p-6 shadow-[8px_8px_0px_0px_#1d2021] text-center",
                    p { class: "font-bold text-xl uppercase", "Disclaimer" }
                    p { class: "font-bold mt-2", "Yes, I am fully aware that some parts of this website might look a bit broken. I am a newbie Rust dev trying to center divs and draw boxes. Please cut me some slack.>.<" }
                }
            }
        }
    }
}

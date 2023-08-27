// Check if the History API is supported
const historyApiSupported = window.history && window.history.pushState;
historyApiSupported && console.log('History API is supported');

// Function to navigate to a route using History API or full page reload
function navigateToRoute(route) {
    if (historyApiSupported) {
        console.log('Navigating to route ' + route);
        // Update the URL using the History API
        window.history.pushState({ route }, null, route);
        Router.handleRoute(route); // Call your route handling logic
    } else {
        // Fallback for browsers without History API
        window.location.href = route; // Perform full page reload
    }
}

// class Button {
//     _el = null;

//     constructor(props) {
//         const node = document.createElement(props.href ? 'a' : 'button');
//         node.className = props.className;
//         if (props.onClick) {
//             node.addEventListener('click', props.onClick);
//         }
//         node.innerText = props.text;
//         this._el = node;
//     }

//     getNode() {
//         return this._el;
//     }
// }

class Router {
    static url = "";
    static routeNode = null;
    static initHTML = "";

    static hydrate() {
        const beforeHTML = Router.initHTML
            .replace(/\s*(<[^>]*>)\s*/g, '$1')
            .replace(/\s+/g, ' ').trim();
        const newHTML = Router.routeNode.innerHTML
            .replace(/\s*(<[^>]*>)\s*/g, '$1')
            .replace(/\s+/g, ' ').trim();

        console.log("HTML before JS: ", beforeHTML);
        console.log("HTML after JS: ", newHTML);

        if (beforeHTML === newHTML) {
            console.log("Hydration completed successfully!");
        } else {
            const modal = document.createElement('div');
            modal.className = "modal";
            const text = document.createElement('p');
            text.innerText = "SSR Hydration Failure!";
            const text2 = document.createElement('pre');
            text2.className = "alert";
            text2.innerText = `Got: ${beforeHTML}`;
            const text3 = document.createElement('pre');
            text3.className = "alert";
            text3.innerText = `Expected: ${newHTML}`;

            modal.appendChild(text);
            modal.appendChild(text2);
            modal.appendChild(text3);
            Router.routeNode.appendChild(modal);
        }
    }

    static handleRoute(route) {
        // Make copy of the current HTML
        Router.initHTML = Router.routeNode.innerHTML;

        Router.routeNode.innerHTML = '';  // Clear existing content

        switch (route) {
            case '/': {
                const node = document.createElement('p');
                node.innerText = "Hello world!";
                const navigator = new Button({
                    text: "Next Page",
                    onClick: function () {
                        navigateToRoute('/data');
                    },
                });
                Router.routeNode.appendChild(node);
                Router.routeNode.appendChild(navigator.getNode());
                // Router.hydrate();
                break;
            }
            case '/data': {
                const typo = new Typography({
                    text: "Lorem Ipsum!",
                });
                const navigator = new Button({
                    text: "Prev Page",
                    onClick: function () {
                        navigateToRoute('/');
                    },
                });
                Router.routeNode.appendChild(typo.getNode());
                Router.routeNode.appendChild(navigator.getNode());
                // Router.hydrate();
                break;
            }
        }
    }
}

window.addEventListener('popstate', function (event) {
    console.log('popstate event: ', event.state);

    // Get the new route from the event state
    const newRoute = event.state?.route;

    if (!newRoute) {
        Router.handleRoute('/');
    } else {
        // Call your route handling logic here
        Router.handleRoute(newRoute);
    }
});

function onPageLoaded() {
    const initialState = history.state;
    const initialRoute = window.location.pathname;

    Router.routeNode = document.getElementById('root');

    if (initialState && initialState.route) {
        // If there's an initial state, handle it based on the state's route
        console.log(`[HYDRATE]: ${initialState.route}`);

        Router.handleRoute(initialState.route);
    } else {
        // Handle the initial route based on the URL
        Router.handleRoute(initialRoute);
    }
}

window.addEventListener('load', onPageLoaded);

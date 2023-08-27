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
            text.innerText = "Hydration failed!";
            modal.appendChild(text);
            Router.routeNode.appendChild(modal);
        }
    }

    static handleRoute(route) {
        Router.routeNode.innerHTML = '';  // Clear existing content

        switch (route) {
            case '/': {
                const node = document.createElement('p');
                node.innerText = "Hello world!";
                Router.routeNode.appendChild(node);
                Router.hydrate();
                break;
            }
            case '/data': {
                const node = document.createElement('p');
                node.innerText = "DATA!";
                const navigator = document.createElement('button');
                navigator.onclick = function () {
                    navigateToRoute('/');
                }
                navigator.innerText = "Prev Page";
                Router.routeNode.appendChild(node);
                Router.routeNode.appendChild(navigator);
                Router.hydrate();
                break;
            }
        }
    }
}

// Event listener for back and forward navigation
window.addEventListener('popstate', function (event) {
    // Get the new route from the event state
    const newRoute = event.state;

    // Call your route handling logic here
    Router.handleRoute(newRoute);
});

window.onload = () => {
    Router.routeNode = document.getElementById('root');
    Router.initHTML = Router.routeNode.innerHTML;
    // const ssrData = document.getElementById("__SSR_DATA__");

    // var fullPath = window.location.href; // Get the full URL
    // var path = new URL(fullPath).pathname; // Extract the pathname from the URL
    // var extractedPath = '/' + path.replace(/^\/+/g, ''); // Remove leading slashes

    // Router.url = extractedPath;
    // navigateToRoute(Router.url);

    const initialRoute = window.location.pathname;
    const initialState = history.state;
    if (initialState) {
        // Use initialState to set up the initial content based on the state
        console.log('Initial state: ', initialState);
        Router.handleRoute(initialRoute);
    } else {
        // Handle the initial route without state
        Router.handleRoute(initialRoute);
    }
};


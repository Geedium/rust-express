class Typography {
    _el = null;

    constructor(props) {
        const node = document.createElement('p');
        node.className = "text";
    node.innerText = props.text; 
        this._el = node;
    }

    getNode() {
        return this._el;
    }
}
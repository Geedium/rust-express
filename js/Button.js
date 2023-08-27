class Button {
    _el = null;

    constructor(props) {
        const node = document.createElement('button');
        node.className = "button-container";
    const text = document.createElement("p");  
  text.innerText = "A Button"; node.appendChild(text); 
        node.addEventListener('click', props.onClick); 
          this._el = node;
    }

    getNode() {
        return this._el;
    }
}
const TERMINAL_STATES = {
  IDLE: "IDLE",
  BROWSING: "BROWSING-HISTORY",
};

export class Terminal {
  constructor(inputEl, outputEl) {
    this.inputEl = inputEl;
    this.outputEl = outputEl;

    this.state = {
      id: TERMINAL_STATES.IDLE
    }

    this.history = [];

    this.inputEl.onkeyup = (e) => {
      switch (e.keyCode) {
        case 13:
          this._handleEnter();
          break;
        
        case 38:
          this._handleArrowUp();
          break;
        
        case 40:
          this._handleArrowDown();
          break;
        
        default:
          if (this.state.id === TERMINAL_STATES.BROWSING) {
            this.state = {
              id: TERMINAL_STATES.IDLE
            }
          }
      }
    }
  }

  onInput() {
    console.log("On Input")
  }

  println() {
    console.log("Printing line")
  }

  scrollToTop() {
    this.println("Scrolling to the top")
  }

  _handleEnter() {
    console.log("Pressing enter")
  }

  _handleArrowUp() {
    console.log("Handling arrow up")
  }

  _handleArrowDown() {
    console.log("Handling arrow down")
  }
}
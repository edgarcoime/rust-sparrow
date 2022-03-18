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
          this._handleEnter(e);
          break;
        
        case 38:
          this._handleArrowUp(e);
          break;
        
        case 40:
          this._handleArrowDown(e);
          break;
        
        default:
          if (this.state.id === TERMINAL_STATES.BROWSING) {
            this.state = {
              id: TERMINAL_STATES.IDLE
            }
          }
      }
    }

    this.onInputHandler = (_line) => void 0;
  }

  onInput(fn) {
    this.onInputHandler = fn;
  }

  println(msg) {
    if (this.outputEl.value) {
      this.outputEl.value += "\n";
    }

    this.outputEl.value += msg;
    this.outputEl.scrollToTop = this.outputEl.scrollHeight;
  }

  scrollToTop() {
    this.inputEl.focus();
    this.outputEl.scrollToTop = 0;
  }

  _handleEnter(event) {
    event.preventDefault();

    const input = this.inputEl.value.trim();
    if (input.length > 0) {
      this.history.push(input);
      this.onInputHandler(input);
    }

    this.inputEl.value = "";

    this.state = {
      id: TERMINAL_STATES.IDLE
    }
    console.log(this.history)
  }

  _handleArrowUp(event) {
    console.log("Handling arrow up")
  }

  _handleArrowDown(event) {
    console.log("Handling arrow down")
  }
}
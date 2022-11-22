import {BaseElement, html, css} from '../static/flow-ux.js';
import * as kaspa from '../kaspa/kaspa.js';

class App extends BaseElement{
    constructor(){
        super();

        this.init();
    }

    async init(){
        const wasm = await kaspa.default('/kaspa/kaspa_bg.wasm');
        console.log("wasm", wasm, kaspa)
        kaspa.start_console_logger();
        //kaspa.show_logs();
        kaspa.bindWorkflow(kaspa, {});
    }
}

App.define("kaspa-app");

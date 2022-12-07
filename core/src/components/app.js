import {FlowApp, html, css} from '[FLOW-UX-PATH]';

const kaspa = window.$workflow$.workflow;

class KaspaApp extends FlowApp{
    constructor(){
        super();
    }
    firstUpdated(...args){
        super.firstUpdated(...args);

        let layout = this.renderRoot.querySelector("workflow-app-layout");
        layout.addEventListener("ready", ()=>{
            this.loadUX();
        })
    }
    async loadUX(){
        this.app = new kaspa.Application()
        console.log("= = = = = LOADING MODULES");
        await this.app.init()
        // await this.app.init(window.location.hash);
        console.log("= = = = = LOADING MODULES DONE");
    }
}

KaspaApp.define("kaspa-app");

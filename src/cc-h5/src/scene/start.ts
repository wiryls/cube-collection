namespace scene {
/////////////////////////////////////////////////////////////////////////////

export class Start extends eui.Component
{
        constructor() {
        super();
        this.addEventListener( eui.UIEvent.COMPLETE, this.uiCompHandler, this );
        this.skinName = "resource/custom_skins/startUISkin.exml";
        }
    private uiCompHandler():void {
        console.log( "\t\tAboutUI uiCompHandler" );
        this.imgbg.source = "loadingbg_png";
        this.mbtnStart.addEventListener( egret.TouchEvent.TOUCH_TAP, this.mbtnHandler, this );
    }
    private imgbg:eui.Image;
    private mbtnStart:eui.Button;

    private mbtnHandler( evt:egret.TouchEvent ):void{
        this.dispatchEventWith( "EVT_LOAD_PAGE", false, "" );
    }
}

/////////////////////////////////////////////////////////////////////////////
}
namespace scene {
/////////////////////////////////////////////////////////////////////////////

export class Start extends eui.Component
{
    private image_background: eui.Image;
    private button_start: eui.Button;

    constructor()
    {
        super();
        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
        this.addEventListener(eui.UIEvent.COMPLETE, this.onUIComplete, this);
        this.skinName = "resource/layouts/StartUI.exml";
    }

    private onUIComplete(): void
    {
        this.button_start.addEventListener(egret.TouchEvent.TOUCH_TAP, this.onStartGame, this);
        Musician.music(utils.Track.BGM_NORMAL);
    }

    private onAddToStage(): void
    {
        this.width  = this.stage.stageWidth;
        this.height = this.stage.stageHeight;
    }

    private onStartGame( evt:egret.TouchEvent ):void
    {
        Director.replace(new scene.World());
        Musician.sound(utils.Track.UI_BUTTON_CLICK);
    }
}

/////////////////////////////////////////////////////////////////////////////
}
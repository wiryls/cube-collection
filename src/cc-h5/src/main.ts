/////////////////////////////////////////////////////////////////////////////

const Director = utils.Director.instance;

/////////////////////////////////////////////////////////////////////////////

class Main extends eui.UILayer
{
    private loadingBg: egret.Bitmap;
    private static readonly Resources: ReadonlyArray<string> =
    [
        "loading", "level", "sound"
    ];

    protected createChildren(): void
    {
        super.createChildren();

        // lifecycle
        egret.lifecycle.addLifecycleListener((context) => {
            // custom lifecycle plugin
        });

        egret.lifecycle.onPause = () => {
            egret.ticker.pause();
        };

        egret.lifecycle.onResume = () => {
            egret.ticker.resume();
        };

        // register
        egret.registerImplementation("eui.IAssetAdapter", new AssetAdapter());
        egret.registerImplementation("eui.IThemeAdapter", new ThemeAdapter());

        // run
        this.onRunning().catch(e => {
            console.error(e);
        });
    }

    private async onRunning()
    {
        try {
            // load config, theme and LoadingUI resources
            await RES.loadConfig("resource/default.res.json", "resource/");
            await new Promise((resolve, reject) => {
                new eui.Theme("resource/default.thm.json", this.stage)
                    .addEventListener(eui.UIEvent.COMPLETE, () => resolve(), this);
            });
            await RES.loadGroup("loading", 0);
            //add bg
            //this.loadingBg = new egret.Bitmap(RES.getRes("loadingbg_png"));
            //this.addChild( this.loadingBg );
            // load all other resources
            const loading = new scene.Loading();
            this.addChild(loading);
            loading.track = Main.Resources;
            for (const group of Main.Resources) {
                await RES.loadGroup(group, 0, loading);
                loading.count();
            }
            this.removeChild(loading);
        } catch (e) {
            throw new Error(e);
        }

        this.onHelloWorld();
    }

    protected onHelloWorld(): void
    {
        Director.main = this;
        Director.push(new scene.World());
    }
}

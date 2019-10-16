/////////////////////////////////////////////////////////////////////////////

const Director = utils.Director.instance;

/////////////////////////////////////////////////////////////////////////////

class Main extends eui.UILayer
{
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
            await RES.loadGroup(Main.Resources[0], 0);
        } catch (e) {
            throw new Error(e);
        }

        // load all other resources
        const loading = new scene.Loading();
        this.addChild(loading);
        loading.track = Main.Resources;
        for (const group of Main.Resources) {
            try {
                await RES.loadGroup(group, 0, loading);
            } catch (e) {
                throw new Error(e);
            }
            loading.count();
        }

        // load HelloWorld Scene
        this.onHelloWorld();
        this.setChildIndex(loading, this.numChildren - 1);

        // transition animation
        egret.Tween.get(loading).to({alpha: 0}, 2000, egret.Ease.circOut).call(() => {
            this.removeChild(loading);
        });
    }

    protected onHelloWorld(): void
    {
        Director.main = this;
        Director.push(new scene.World());
    }
}

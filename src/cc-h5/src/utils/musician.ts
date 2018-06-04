namespace utils {
/////////////////////////////////////////////////////////////////////////////

import Sound   = egret.Sound;
import Channel = egret.SoundChannel;

export namespace Track
{
    export const BGM_NORMAL = "sound_music_normal";
    export const BGM_HEAVY  = "sound_music_heavy";

    export const CUBE_CONTROL = "sound_effect_note_move";
    export const CUBE_CONNECT = "sound_effect_note_connect";
    
    export const LEVEL_ENTER = "sound_effect_note_01";
    export const LEVEL_CLEAR = "sound_effect_level_clear";
}

export class Musician
{
    private playing = new Map<string, Channel>();

    private constructor() { }
    private static instance_: Musician;
    public static get instance()
    {
        return this.instance_ || (this.instance_ = new this());
    }

    public sound(name: string): void
    {
        const sound = this.tryToFind(name);
        if (sound !== undefined) {
            sound.play(0, 1).volume = 0.6;
        } else {
            console.error("Musician: Cannot find sound", name);
        }
    }

    public music(name: string): void
    {
        this.tryToStop(name);
        const sound = this.tryToFind(name);
        if (sound !== undefined) {
            const channel = sound.play(0, 0);
            channel.volume = 0.3;
            this.playing.set(name, channel);
        } else {
            console.error("Musician: Cannot find music", name);
        }
    }

    public stop(name: string): void
    {
        if (this.tryToStop(name) === false)
            console.error("Musician: Cannot find and stop", name);
    }

    private tryToFind(name: string): Sound|undefined
    {
        return(RES.hasRes(name))
            ? (RES.getRes(name) as Sound)
            : (undefined)
            ;
    }

    private tryToStop(name: string): boolean
    {
        if (this.playing.has(name)) {
            const channel = this.playing.get(name);
            if (channel !== undefined) {
                this.playing.delete(name);
                channel.stop();
                return true;
            }
        }
        return false;
    }
}

/////////////////////////////////////////////////////////////////////////////
}
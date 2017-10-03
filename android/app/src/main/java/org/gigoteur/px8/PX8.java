package org.gigoteur.px8;

import org.libsdl.app.SDLActivity;

public class PX8 extends SDLActivity
{
	@Override
	protected String[] getLibraries() {
		return new String[] {
			"SDL2",
			"px8"
		};
	}
    
}
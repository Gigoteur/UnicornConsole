package org.gigoteur.unicorn;

import org.libsdl.app.SDLActivity;

public class Unicorn extends SDLActivity
{
	@Override
	protected String[] getLibraries() {
		return new String[] {
			"SDL2",
			"uc"
		};
	}
    
}
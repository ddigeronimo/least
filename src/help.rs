pub const HELP_MESSAGE: &str = r#"
        :::       ::::::::::    :::     ::::::::::::::::::: 
        :+:       :+:         :+: :+:  :+:    :+:   :+:     
        +:+       +:+        +:+   +:+ +:+          +:+     
        +#+       +#++:++#  +#++:++#++:+#++:++#++   +#+     
        +#+       +#+       +#+     +#+       +#+   +#+     
        #+#       #+#       #+#     #+##+#    #+#   #+#     
        #######################     ### ########    ###     

                      © 2020 Dylan DiGeronimo

                Usage: least [-h, --help | filename]

                   Controls:
                       - q - Quit
                       - j, Down - Down one line
                       - k, Up - Up one line
                       - d, PgDn - Down half a screen
                       - u, PgUp - Up half a screen
                       - g - Jump to top of file
                       - / - Forward search
                       - ? - Backward search
                       - n - Next search result
                       - N - Last search result
                       - o - Open a new file
                       - h - Open help screen
"#;
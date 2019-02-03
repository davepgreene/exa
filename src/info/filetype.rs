//! Tests for various types of file (video, image, compressed, etc).
//!
//! Currently this is dependent on the file’s name and extension, because
//! those are the only metadata that we have access to without reading the
//! file’s contents.

use ansi_term::Style;

use fs::File;
use output::file_name::FileColours;
use output::icons::FileIcon;


#[derive(Debug, Default, PartialEq)]
pub struct FileExtensions;

impl FileExtensions {

    /// An “immediate” file is something that can be run or activated somehow
    /// in order to kick off the build of a project. It’s usually only present
    /// in directories full of source code.
    fn is_immediate(&self, file: &File) -> bool {
        file.name.to_lowercase().starts_with("readme") || file.name_is_one_of( &[
            "Makefile", "Cargo.toml", "SConstruct", "CMakeLists.txt",
            "build.gradle", "Rakefile", "Gruntfile.js",
            "Gruntfile.coffee", "BUILD", "WORKSPACE", "build.xml"
        ])
    }

    fn is_android(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "android", "apk", "gradle",
        ])
    }

    fn is_apple(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "apple", "ds_store", "localized", "plist",
        ])
    }

    fn is_music(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "aac", "m4a", "mp3", "ogg", "wma", "mka", "opus",
        ])
    }

    // Lossless music, rather than any other kind of data...
    fn is_lossless(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "alac", "ape", "flac", "wav",
        ])
    }

    fn is_compressed(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "7z", "a", "ar", "arj", "bz2", "cpio", "gz", "lrz", "lz", "lzma",
            "lzo", "rar", "s7z", "sz", "tar", "tgz", "xz", "zip", "zipx",
            "zoo", "zpaq", "zz", "z",
        ])
    }

    fn is_conf(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "conf", "editorconfig",
        ])
    }

    fn is_crypto(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "asc", "enc", "gpg", "pgp", "sig", "signature", "pfx", "p12",
        ])
    }


    fn is_css(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "css", "scss",
        ])
    }

    fn is_doc(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "doc", "docx", "gdoc",
        ])
    }

    fn is_ebook(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "ebook", "epub", "mobi", "azw",
        ])
    }

    fn is_font(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "eot", "otf", "ttf", "woff", "woff2",
        ])
    }

    fn is_git(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "git", "gitconfig", "gitignore", "gitignore_global", "gitattributes",
        ])
    }

    fn is_hs(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "hs", "lhs",
        ])
    }

    fn is_html(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "html", "htm", "jhtm", "mht", "eml",
        ])
    }

    fn is_image(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "dmg", "iso", "bin", "nrg", "qcow", "sparseimage", "toast", "vcd",
            "vmdk", "pkg",
        ])
    }

    fn is_java(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "java", "jar", "war",
        ])
    }

    fn is_json(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "json", "properties",
        ])
    }

    fn is_jsx(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "jsx", "tsx",
        ])
    }

    fn is_md(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "md", "license", "markdown", "mkd", "rdoc", "readme",
        ])
    }

    fn is_package(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "deb", "rpm", "jad", "jar", "cab", "pak", "pk3", "vdf", "bsp",
        ])
    }

    fn is_picture(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "bmp", "tiff", "tif", "cdr", "gif", "ico", "jpeg", "jpg", "nth",
            "png",  "xpm", "eps", "epsf", "drw", "svg", "raw",
        ])
    }

    fn is_ppt(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "pps", "ppt", "pptx", "ppts", "pptxm", "pptsm", "gslides",
        ])
    }

    fn is_py(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "py", "ipynb", "pyc",
        ])
    }

    fn is_r(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "r", "rdata", "rds",
        ])
    }

    fn is_rb(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "gemfile", "gemspec", "guardfile", "lock",
            "procfile", "rakefile", "rspec", "rspec_parallel",
            "rspec_status", "ru", "rb"
        ])
    }

    fn is_rubydoc(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "rubydoc", "erb", "slim",
        ])
    }

    fn is_shell(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "awk", "bash", "sed", "sh", "zsh", "ahk", "bash_history",
            "bash_profile", "bashrc", "fish", "zsh-theme", "zshrc",
            "zsh_history", "lesshst",
        ])
    }

    fn is_styl(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "styl", "stylus",
        ])
    }

    fn is_tex(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "tex", "cls",
        ])
    }

    fn is_video(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "avi", "divx", "ifo", "m2v", "m4v", "mkv", "mov", "mp4", "mpeg",
            "mpg", "ogm", "rmvb", "sample", "wmv", "3g2", "3gp", "gp3", "webm",
            "gp4", "asf", "flv", "ogv", "f4v", "vob"
        ])
    }

    fn is_vim(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "vim", "vimrc", "viminfo"
        ])
    }

    fn is_windows(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "bat", "exe", "ini", "com",
        ])
    }

    fn is_xls(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "ods", "xla", "xls", "xlsx", "xlsxm", "xltm", "xltx", "csv", "gsheet",
        ])
    }

    fn is_xml(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "xml", "xul",
        ])
    }

    fn is_yml(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "yml", "yaml",
        ])
    }

    fn is_document(&self, file: &File) -> bool {
        file.extension_is_one_of( &[
            "djvu", "doc", "docx", "docm", "dvi", "eml", "eps", "fotd",
            "odb", "odt", "pdf", "ppt", "pptx", "rtf", "ps",
            "xls", "xlsx",
        ])
    }

    fn is_temp(&self, file: &File) -> bool {
        file.name.ends_with('~')
            || (file.name.starts_with('#') && file.name.ends_with('#'))
            || file.extension_is_one_of( &[ "tmp", "swp", "swo", "swn", "bak", "bk" ])
    }

    fn is_compiled(&self, file: &File) -> bool {
        if file.extension_is_one_of( &[ "class", "elc", "hi", "o", "pyc" ]) {
            true
        }
        else if let Some(dir) = file.parent_dir {
            file.get_source_files().iter().any(|path| dir.contains(path))
        }
        else {
            false
        }
    }
}

impl FileColours for FileExtensions {
    fn colour_file(&self, file: &File) -> Option<Style> {
        use ansi_term::Colour::*;

        Some(match file {
            f if self.is_android(f)     => Fixed(40).normal(),
            f if self.is_apple(f)       => Fixed(239).normal(),
            f if self.is_music(f)       => Fixed(92).normal(),
            f if self.is_lossless(f)    => Fixed(93).normal(),
            f if self.is_compressed(f)  => Red.normal(),
            f if self.is_crypto(f)      => Fixed(192).normal(),
            f if self.is_css(f)         => Fixed(125).normal(),
            f if self.is_doc(f)         => Fixed(111).normal(),
            f if self.is_ebook(f)       => Fixed(141).normal(),
            f if self.is_font(f)        => Fixed(66).normal(),
            f if self.is_git(f)         => Fixed(197).normal(),
            f if self.is_hs(f)          => Fixed(81).normal(),
            f if self.is_html(f)        => Fixed(125).normal(),
            f if self.is_image(f)       => Fixed(40).normal(),
            f if self.is_java(f)        => Fixed(40).normal(),
            f if self.is_json(f)        => Fixed(178).normal(),
            f if self.is_jsx(f)         => Fixed(178).normal(),
            f if self.is_md(f)          => Fixed(184).normal(),
            f if self.is_package(f)     => Fixed(40).normal(),
            f if self.is_picture(f)     => Fixed(133).normal(),
            f if self.is_ppt(f)         => Fixed(166).normal(),
            f if self.is_py(f)          => Fixed(41).normal(),
            f if self.is_r(f)           => Fixed(49).normal(),
            f if self.is_rb(f)          => Fixed(1).normal(),
            f if self.is_rubydoc(f)     => Fixed(1).normal(),
            f if self.is_shell(f)       => Fixed(172).normal(),
            f if self.is_tex(f)         => Fixed(184).normal(),
            f if self.is_video(f)       => Fixed(114).normal(),
            f if self.is_vim(f)         => Fixed(1).normal(),
            f if self.is_windows(f)     => Fixed(172).normal(),
            f if self.is_xls(f)         => Fixed(112).normal(),
            f if self.is_xml(f)         => Fixed(178).normal(),
            f if self.is_yml(f)         => Fixed(178).normal(),
            f if self.is_immediate(f)   => Yellow.bold().underline(),
            f if self.is_document(f)    => Fixed(105).normal(),
            f if self.is_temp(f)        => Fixed(244).normal(),
            f if self.is_compiled(f)    => Fixed(137).normal(),
            _                           => return None,
        })
    }
}

impl FileIcon for FileExtensions {
    fn icon_file(&self, file: &File) -> Option<char> {
        use output::icons::Icons;

        Some(match file {
            f if self.is_android(f)    => Icons::Android.value(),
            f if self.is_apple(f)      => Icons::Apple.value(),
            f if self.is_music(f)      => Icons::Audio.value(),
            f if self.is_lossless(f)   => Icons::Audio.value(),
            f if self.is_compressed(f) => Icons::Compressed.value(),
            f if self.is_conf(f)       => Icons::Conf.value(),
            f if self.is_crypto(f)     => Icons::Crypto.value(),
            f if self.is_css(f)        => Icons::Css.value(),
            f if self.is_doc(f)        => Icons::Doc.value(),
            f if self.is_ebook(f)      => Icons::Ebook.value(),
            f if self.is_font(f)       => Icons::Font.value(),
            f if self.is_git(f)        => Icons::Git.value(),
            f if self.is_hs(f)         => Icons::Hs.value(),
            f if self.is_html(f)       => Icons::Html.value(),
            f if self.is_image(f)      => Icons::Image.value(),
            f if self.is_java(f)       => Icons::Java.value(),
            f if self.is_json(f)       => Icons::Json.value(),
            f if self.is_jsx(f)        => Icons::Jsx.value(),
            f if self.is_md(f)         => Icons::Md.value(),
            f if self.is_package(f)    => Icons::Package.value(),
            f if self.is_picture(f)    => Icons::Picture.value(),
            f if self.is_ppt(f)        => Icons::Ppt.value(),
            f if self.is_py(f)         => Icons::Py.value(),
            f if self.is_r(f)          => Icons::R.value(),
            f if self.is_rb(f)         => Icons::Rb.value(),
            f if self.is_rubydoc(f)    => Icons::Rubydoc.value(),
            f if self.is_shell(f)      => Icons::Shell.value(),
            f if self.is_styl(f)       => Icons::Styl.value(),
            f if self.is_tex(f)        => Icons::Tex.value(),
            f if self.is_video(f)      => Icons::Video.value(),
            f if self.is_vim(f)        => Icons::Vim.value(),
            f if self.is_windows(f)    => Icons::Windows.value(),
            f if self.is_xls(f)        => Icons::Xls.value(),
            f if self.is_xml(f)        => Icons::Xml.value(),
            f if self.is_yml(f)        => Icons::Yml.value(),
            _ => return None,
        })
    }
}

const CleanCSS = require("clean-css");

module.exports = function(eleventyConfig) {
    eleventyConfig.addPassthroughCopy({ "../qr_codes": "qr-codes" });

    eleventyConfig.addFilter("cssmin", function(code) {
        return new CleanCSS({}).minify(code).styles;
    });

    return {
        dir: {
            input: 'src',
            output: 'build',
            data: '../../data',
            layouts: 'layouts',
            includes: 'includes',
        },
    }
};

const qrFolder = '../qr_codes/';
const fs = require('fs');

module.exports = async function() {
    console.log( "Generating QR code data" );
    return {
        unsigned: get_qr_info("unsigned"),
        signed: get_qr_info("signed"),

    };
};


function get_qr_info(subfolder) {
    return fs.readdirSync(qrFolder + subfolder, {withFileTypes: true})
        .filter(item => !item.isDirectory())
        .reduce((acc, item) => {
            const filename = item.name.split('.')[0]
            console.log(filename)
            const [chain, kind, version] = filename.split('_')
            if (!chain || !kind || !version) {
                return acc
            }
            if (!acc[chain]) {
                acc[chain] = []
            }
            acc[chain].push({
                path: `/qr-codes/${subfolder}/${item.name}`,
                kind: kind,
                version: version
            })
            return acc
        }, {})
}

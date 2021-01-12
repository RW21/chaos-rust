WebAssembly.instantiateStreaming(fetch('main.wasm'))
    .then(obj => {
        obj.instance.exports.run()
        // const byteArray = new Uint8ClampedArray(memory.buffer, 0, 1024 * 1024 * 4)
        const byteArray = Uint8Array
        const image = new ImageData(byteArray, 1024, 1024)
        const canvas = document.getElementById('canvas')
        const ctx = canvas.getContext('2d')
        ctx.putImageData(image, 0, 0)
        document.getElementById('loader').style.visibility = 'hidden'
    })

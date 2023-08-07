<script>
    import Colorbox from "$lib/components/colorbox.svelte";
    import CustomButton from "$lib/components/custom_button.svelte";

    let picture_canvas 
    let inputFile

    let first_color = "white"
    let second_color = "white"
    let third_color = "white"

    let selected_sample = 1


    function changeSample(num){
	selected_sample = num
    }

    function picture_uploaded(e){
	let img = new Image()
	img.onload = () => {
	    //TODO scale image in canvas: same aspect ratio
	    let ctx = picture_canvas.getContext("2d")
	    ctx.drawImage(img, 
		0, 0, img.width, img.height,
		0, 0, picture_canvas.width, picture_canvas.height)

	}
	img.src = URL.createObjectURL(e.target.files[0])
    }

    function pick_color(e){
	let x = e.offsetX
	let y = e.offsetY
	let ctx = picture_canvas.getContext("2d")
	let data = ctx.getImageData(x, y, 50, 50).data
	let red = data[0]
	let green = data[1]
	let blue = data[2]

	
	let hex = "#" + red.toString(16) + green.toString(16) + blue.toString(16)
	if(selected_sample == 1)
	    first_color = hex
	else if(selected_sample == 2)
	    second_color = hex
	else third_color = hex
    }

</script>

<div id="container">
    <canvas
	on:click={pick_color}
	bind:this={picture_canvas}>
    </canvas>
    <br>
    <Colorbox 
	on:click={() => changeSample(1)}
	--color={first_color}
	></Colorbox>
    <Colorbox 
	on:click={() => changeSample(2)}
	--color={second_color}
	></Colorbox>
    <Colorbox 
	on:click={() => changeSample(3)}
	--color={third_color}
	></Colorbox>
    <br>
    <input type="file"
	   bind:this={inputFile}
	   on:change={picture_uploaded}
    >
    <button on:click={ () => inputFile.click()}>Upload</button>
    <div>
	<CustomButton
	    --btnColor="#ce2c2c"
	    >Cancel</CustomButton>
    	<CustomButton
	    --btnColor="#51a44c"
	    >Confirm</CustomButton>
    </div>
</div>
<style>
    input[type="file"]{
	display: none;
    }
    #container {
	margin: auto;
	text-align: center;
    }
    input {
	margin: 10px;
	display: inline;
    }
    canvas {
	background: white;
	border: 1px solid black;

	margin: 10px;
    }
</style>


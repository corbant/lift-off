<script>
	import { ProgressBar } from 'carbon-components-svelte'
	let connected = false;
</script>


{#if connected}
	<div>
		<img src="connected.svg" alt="Connected"/>
		<h3>Connected and ready to go!</h3>
	</div>
{:else}
	<div>
		<img src="waiting.svg" alt="Waiting"/>
		<h3>Waiting for connection to the ground station...</h3>
		<ProgressBar helperText="Waiting" />
	</div>
{/if}




<style>
	div {
		margin: 0 auto;
		max-width: 700px;
	}

	img {
		width: 100%;
		height: 40vh;
	}

	h3 {
		text-align: center;
		margin-top: 24px;
		margin-bottom: 32px;
	}

	@media (max-height: 420px) {
		img {
			display: none;
		}
	}
</style>
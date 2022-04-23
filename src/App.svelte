<script>
	import { Header, HeaderUtilities, HeaderAction, SkipToContent, SideNav, SideNavItems, SideNavLink, ProgressIndicator, ProgressStep, TileGroup, RadioTile, HeaderPanelLinks } from 'carbon-components-svelte'
	import UsbIcon from 'svelte-material-icons/Usb.svelte'
	import 'carbon-components-svelte/css/all.css'

	let sideNavOpen = false;
	let progressIndex = 1;
	let connectedDevice = null;
</script>

<main>
	<Header company="LSI" platformName="Mission Control" bind:isSideNavOpen={sideNavOpen}>
		<svelte:fragment slot="skip-to-content">
			<SkipToContent />
		</svelte:fragment>
		<HeaderUtilities>
			<div class="device-indicator">
				<UsbIcon color="white" size="26"/>
				<p>{connectedDevice == null ? "No device connected" : connectedDevice}</p>
			</div>
		</HeaderUtilities>
	</Header>

	<SideNav bind:isOpen={sideNavOpen}>
		<div class="progress">
			<ProgressIndicator vertical bind:currentIndex={progressIndex}>
				<ProgressStep complete label="Connection"/>
				<ProgressStep label="Preflight"/>
				<ProgressStep label="Test 3"/>
			</ProgressIndicator>
		</div>
	</SideNav>
</main>

<style>
	.progress {
		margin-left: 12px;
		margin-top: 16px;
	}

	p {
		color: white
	}

	.device-indicator {
		display: flex;
		margin: auto 16px;
	}
</style>
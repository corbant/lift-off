<script>
	import { Header, HeaderUtilities, HeaderAction, SkipToContent, SideNav, SideNavItems, SideNavLink, ProgressIndicator, ProgressStep, TileGroup, RadioTile, HeaderPanelLinks, Content, Theme } from 'carbon-components-svelte'
	import { Router, Link, Route } from 'svelte-routing'
	import Preflight from './routes/Preflight.svelte'
	import Connection from './routes/Connection.svelte'
	import UsbIcon from 'svelte-material-icons/Usb.svelte'
	import 'carbon-components-svelte/css/all.css'

	let sideNavOpen = false;
	let progressIndex = 1;
	let connectedDevice = null;
</script>

<Theme theme="white"/>

<main>
	<Router>
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
					<Link to="/">
						<ProgressStep complete label="Connection"/>
					</Link>
					<Link to="/preflight">
						<ProgressStep complete label="Preflight"/>
					</Link>
					<Link to="/">
						<ProgressStep label="Test 3"/>
					</Link>
				</ProgressIndicator>
			</div>
		</SideNav>

		<Content>
			<Route path="/" component={Connection}/>
			<Route path="/preflight" component={Preflight} />
		</Content>
	</Router>
</main>

<style>
	.progress {
		margin-left: 12px;
		margin-top: 16px;
		background-color: inherit;
	}

	p {
		color: white
	}

	.device-indicator {
		display: flex;
		margin: auto 16px;
	}
</style>
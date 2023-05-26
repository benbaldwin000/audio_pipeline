import { StyleSheet, View } from 'react-native';
import { MediaGrid, PageHeader, PlayingBar } from '../components';

const styles = StyleSheet.create({
	container: {
		backgroundColor: '#222432',
		flexGrow: 1,
	},
});

export const LibraryView = () => {
	return (
		<View style={styles.container}>
			<PageHeader title='Library' />
			<MediaGrid />
			<PlayingBar />
		</View>
	);
};

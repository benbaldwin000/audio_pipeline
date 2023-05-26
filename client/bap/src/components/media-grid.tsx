import { MediaTile } from './media-tile';
import { StyleSheet, View } from 'react-native';

const styles = StyleSheet.create({
	container: {
		display: 'flex',
		flexDirection: 'row',
		flexGrow: 1,
		flexWrap: 'nowrap',
		padding: 12,
		columnGap: 12,
	},
	column: {
		display: 'flex',
		flex: 1,
	},
});

export const MediaGrid = () => {
	return (
		<View style={styles.container}>
			<View style={styles.column}>
				<MediaTile />
			</View>
			<View style={styles.column}>
				<MediaTile />
			</View>
		</View>
	);
};

import { Image, StyleSheet, Text, View } from 'react-native';

const styles = StyleSheet.create({
	container: {},
	cover: {
		aspectRatio: 1,
		width: '100%',
		resizeMode: 'center',
		marginBottom: 8,
		borderRadius: 6,
	},
	title: {
		color: '#fff',
		fontWeight: '700',
		fontSize: 20,
		flexShrink: 1,
		overflow: 'hidden',
		textAlign: 'center',
	},
});

type MediaTileProps = {};

export const MediaTile = ({}: MediaTileProps) => {
	return (
		<View style={styles.container}>
			<Image source={{ uri: 'https://f4.bcbits.com/img/a1501995349_16.jpg' }} style={styles.cover} />
			<Text style={styles.title} numberOfLines={1}>
				The Way of Ging
			</Text>
		</View>
	);
};

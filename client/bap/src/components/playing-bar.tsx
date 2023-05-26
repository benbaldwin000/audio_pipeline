import Feather from '@expo/vector-icons/Feather';
import { StyleSheet, View, Image, Text } from 'react-native';

const styles = StyleSheet.create({
	container: {
		position: 'absolute',
		left: 18,
		right: 18,
		bottom: 24,
		backgroundColor: '#151517',
		borderRadius: 12,

		overflow: 'hidden',
	},
	infoContainer: {
		paddingVertical: 12,
		paddingHorizontal: 12,
		display: 'flex',
		flexDirection: 'row',
		alignItems: 'center',
	},
	cover: {
		width: 45,
		aspectRatio: 1,
		borderRadius: 2,
		marginRight: 12,
	},
	title: {
		color: '#fff',
		fontWeight: '700',
		fontSize: 16,
	},
	subtitle: {
		color: '#9495a5',
		fontSize: 16,
	},
	controlsContainer: {
		position: 'absolute',
		top: 0,
		right: 0,
		bottom: 0,
		display: 'flex',
		justifyContent: 'center',
		paddingRight: 16,
		paddingLeft: 8,
	},
	progress: {
		height: 4,
		width: '80%',
		backgroundColor: '#fff',
	},
});

export const PlayingBar = () => {
	return (
		<View style={styles.container}>
			<View style={styles.infoContainer}>
				<Image source={{ uri: 'https://f4.bcbits.com/img/a1501995349_16.jpg' }} style={styles.cover} />
				<View>
					<Text style={styles.title}>Electric Prince</Text>
					<Text style={styles.subtitle}>Frank Dukes • Instumental</Text>
				</View>
				<View style={styles.controlsContainer}>
					<Feather name='pause' size={30} color='#fff' />
				</View>
			</View>

			<View style={styles.progress} />
		</View>
	);
};

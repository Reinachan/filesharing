const files = document.querySelectorAll('body > ul > li')!;

for (const file of files) {
	const button = file.querySelector('button')!;
	const span = file.querySelector('.saved-name')!;

	button.addEventListener('click', function (e) {
		e.preventDefault();

		let formData = new FormData();

		formData.append('delete', span.innerHTML);

		fetch('/delete', {
			method: 'POST',
			body: formData,
		})
			.then(function (data) {
				if (!data.ok) throw 'failed';
				button.innerHTML = '✓';
				file.classList.add('deleted');
			})
			.catch(function () {
				button.innerHTML = '𐄂';
			});
	});
}

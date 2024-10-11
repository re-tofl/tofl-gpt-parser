use rouille::Response;

pub fn handle_page(request: &rouille::Request) -> rouille::Response {
    return Response::html(r#"
                        <!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Парсер</title>
</head>
<body>
    <h1>Введите данные</h1>

    <form id="dataForm">
                <p3>TRS:</p3>
        <div>
            <textarea id="trs" name="trs" rows="4" cols="50" required></textarea>
        </div>
        <br>
                    <p3>Interpretation:</p3>
        <div>
            <textarea id="interpretation" name="interpretation" rows="4" cols="50" required></textarea>
        </div>
        <br>
        <button type="submit">Submit</button>
    </form>
    <br>
    <div id="result"></div>

    <script>
        document.getElementById('dataForm').addEventListener('submit', function(event) {
            event.preventDefault();

            const trs = document.getElementById('trs').value;
            const interpretation = document.getElementById('interpretation').value;

            const data = {
                TRS: trs,
                Interpretation: interpretation
            };

            fetch('/parse', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(data)
            })
            .then(response => response.json())
            .then(result => {
                const resultDiv = document.getElementById('result');
                resultDiv.innerHTML = `
                    <strong>Результат:</strong> <pre>${JSON.stringify(result, null, 2)}</pre> <br>
                `;
            })
            .catch(error => {
                console.error('Error:', error);
                document.getElementById('result').innerHTML = 'An error occurred during form submission.';
            });
        });
    </script>
</body>
</html>
"#)
}
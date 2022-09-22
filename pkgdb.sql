CREATE TABLE Packages (
    --name of the github repo
    PackageName varchar(255),
    --github username of the uploader
    PackageUploaderName varchar(255),
    --what language the package was written in
    PackageLanguage varchar(255)
);

INSERT INTO Packages (PackageName,PackageUploaderName,PackageLanguage)
VALUES ("unitytergen","RK33DV","C#(pure)");
INSERT INTO Packages (PackageName,PackageUploaderName,PackageLanguage)
VALUES ("Rox","muppi090909","Rust(pure)");

-- print the values
SELECT * FROM Packages;
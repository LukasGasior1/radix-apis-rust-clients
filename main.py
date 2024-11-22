from cli_command_parser import Command, Option, main
from tempfile import TemporaryDirectory, NamedTemporaryFile
from typing import Any, TypedDict, Literal, Self, Iterator, cast
import toml
import urllib.request
import subprocess
import shutil
import yaml
import copy
import os


class NoAliasDumper(yaml.SafeDumper):
    def ignore_aliases(self, data):
        return True


def download_openapi_generator(directory: str, version: str) -> str:
    """
    Downloads the OpenAPI generator CLI of a given version to a given directory
    and returns the path to the generator's CLI.
    """
    file_name: str = f"openapi-generator-cli-{version}.jar"
    file_path: str = os.path.join(directory, file_name)
    url: str = (
        "https://search.maven.org/remotecontent?filepath=org/openapitools/"
        f"openapi-generator-cli/{version}/{file_name}"
    )
    urllib.request.urlretrieve(url, file_path)
    return file_path


def download_gateway_api_spec(directory: str, branch: str = "main") -> str:
    """
    Downloads the Gateway API's OpenAPI spec of a specified branch to a specific
    directory and returns its path.
    """
    file_name: str = f"gateway-api-schema.yaml"
    file_path: str = os.path.join(directory, file_name)
    url: str = (
        f"https://raw.githubusercontent.com/radixdlt/babylon-gateway/{branch}"
        "/src/RadixDlt.NetworkGateway.GatewayApi/gateway-api-schema.yaml"
    )
    urllib.request.urlretrieve(url, file_path)
    return file_path


def download_core_api_spec(directory: str, branch: str = "main") -> str:
    """
    Downloads the Core API's OpenAPI spec of a specified branch to a specific
    directory and returns its path.
    """
    file_name: str = f"gateway-api-schema.yaml"
    file_path: str = os.path.join(directory, file_name)
    url: str = (
        f"https://raw.githubusercontent.com/radixdlt/babylon-node/{branch}"
        "/core-rust/core-api-server/core-api-schema.yaml"
    )
    urllib.request.urlretrieve(url, file_path)
    return file_path


def fix_discriminated_unions_in_spec(
    input_spec: dict[Any, Any]
) -> dict[Any, Any]:
    """
    Fixes the discriminated unions in the OpenAPI spec to allow for them to be
    generated correctly.
    """
    spec: dict[Any, Any] = copy.deepcopy(input_spec)

    # Applying fixes to the discriminated unions to allow them to be generated
    # properly.
    for key, value in spec["components"]["schemas"].items():
        # An item was discovered that is a discriminated union.
        if "discriminator" in value:
            # Cache the required and properties fields
            required: list[str] = value["required"]
            properties: dict[str, Any] = value["properties"]

            # Delete the "required" and "properties" fields
            del value["required"]
            del value["properties"]

            # Get the property name of the enum
            tag_name: str = value["discriminator"]["propertyName"]

            # Add a oneOf field to the discriminated union of each one of the
            # variants.
            value["oneOf"] = [
                {"$ref": value}
                for value in value["discriminator"]["mapping"].values()
            ]

            # Finished making changes to the discriminated union, write it back
            # to the spec.
            spec["components"]["schemas"][key] = value

            # Iterate over the path of each variant.
            for type_path in value["discriminator"]["mapping"].values():
                variant_name: str = type_path.split("/")[-1]

                # Get the spec of the variant.
                variant_spec: dict[str, Any] = spec["components"]["schemas"][
                    variant_name
                ]

                # Instead of the variant being defined as an "allOf" define it
                # just using its type.
                variant_spec: dict[str, Any] = [
                    item
                    for item in variant_spec["allOf"]
                    if key not in item.get("$ref", "")
                ][0]

                # Add all "required" fields from the enum to the variant that
                # are not the tag
                variant_spec["required"] = variant_spec.get("required", []) + [
                    item for item in required if item != tag_name
                ]
                variant_spec["properties"] = variant_spec.get("properties", {})
                variant_spec["properties"].update(
                    {k: v for k, v in properties.items() if k != tag_name}
                )

                # Write the variant spec back to the spec
                spec["components"]["schemas"][variant_name] = variant_spec

    return spec


class RustOpenApiGeneratorConfiguration(TypedDict):
    avoid_boxed_models: bool
    """If set, Box<T> will not be used for models"""

    best_fit_int: bool
    """Use best fitting integer type where minimum or maximum is set"""

    enum_name_suffix: str
    """Suffix that will be appended to all enum names."""

    hide_generation_timestamp: bool
    """Hides the generation timestamp when files are generated."""

    library: Literal["hyper", "reqwest"]
    """library template (sub-template) to use."""

    package_name: str
    """Rust package name (convention: lowercase)."""

    package_version: str
    """Rust package version."""

    prefer_unsigned_int: bool
    """Prefer unsigned integers where minimum value is >= 0"""

    support_async: bool
    """If set, generate async function call instead. This option is for 'reqwest' library only"""

    support_middleware: bool
    """If set, add support for reqwest-middleware. This option is for 'reqwest' library only"""

    support_multiple_responses: bool
    """If set, return type wraps an enum of all possible 2xx schemas. This option is for 'reqwest' library only"""

    use_single_request_parameter: bool
    """Setting this property to true will generate functions with a single argument containing all API endpoint parameters instead of one argument per parameter."""


class RustOpenApiGenerator:
    """
    A programmatic generator of Rust clients from the OpenAPI specification.
    """

    generator_configurations: RustOpenApiGeneratorConfiguration = {
        "avoid_boxed_models": False,
        "best_fit_int": False,
        "enum_name_suffix": "",
        "hide_generation_timestamp": True,
        "library": "reqwest",
        "package_name": "openapi",
        "package_version": "1.0.0",
        "prefer_unsigned_int": False,
        "support_async": True,
        "support_middleware": False,
        "support_multiple_responses": False,
        "use_single_request_parameter": False,
    }

    def __init__(self) -> None:
        pass

    def avoid_boxed_models(self, setting: bool) -> Self:
        self.generator_configurations["avoid_boxed_models"] = setting
        return self

    def best_fit_int(self, setting: bool) -> Self:
        self.generator_configurations["best_fit_int"] = setting
        return self

    def enum_name_suffix(self, setting: str) -> Self:
        self.generator_configurations["enum_name_suffix"] = setting
        return self

    def hide_generation_timestamp(self, setting: bool) -> Self:
        self.generator_configurations["hide_generation_timestamp"] = setting
        return self

    def library(self, setting: Literal["hyper", "reqwest"]) -> Self:
        self.generator_configurations["library"] = setting
        return self

    def package_name(self, setting: str) -> Self:
        self.generator_configurations["package_name"] = setting
        return self

    def package_version(self, setting: str) -> Self:
        self.generator_configurations["package_version"] = setting
        return self

    def prefer_unsigned_int(self, setting: bool) -> Self:
        self.generator_configurations["prefer_unsigned_int"] = setting
        return self

    def support_async(self, setting: bool) -> Self:
        self.generator_configurations["support_async"] = setting
        return self

    def support_middleware(self, setting: bool) -> Self:
        self.generator_configurations["support_middleware"] = setting
        return self

    def support_multiple_responses(self, setting: bool) -> Self:
        self.generator_configurations["support_multiple_responses"] = setting
        return self

    def use_single_request_parameter(self, setting: bool) -> Self:
        self.generator_configurations["use_single_request_parameter"] = setting
        return self

    def generate(
        self,
        spec: dict[Any, Any],
        generator_path: str,
        path: str,
        delete_existing_files: bool,
    ):
        # Delete any files that exist in that directory if that option is
        # configured without deleting the directory itself.
        if delete_existing_files and os.path.exists(path):
            for item in os.listdir(path):
                item_path: str = os.path.join(path, item)
                if os.path.isfile(item_path):
                    os.remove(item_path)
                else:
                    shutil.rmtree(item_path)

        # Create the directory if it doesn't already exist.
        if not os.path.exists(path):
            os.mkdir(path)

        # Write the Schema to a temporary file that is deleted after this function
        # is executed.
        with NamedTemporaryFile("w") as temporary_file:
            yaml.dump(spec, temporary_file, Dumper=NoAliasDumper)

            # Generate the client
            subprocess.run(
                [
                    # Execute the following Java program
                    "java",
                    "-jar",
                    generator_path,
                    "generate",
                    # Generate Rust Bindings
                    "-g",
                    "rust",
                    # The Input is the Schema file
                    "-i",
                    temporary_file.name,
                    # The output is the the directory specified by the caller.
                    "-o",
                    path,
                    # Specifying the configuration
                    RustOpenApiGenerator.__config_to_cli_params(
                        self.generator_configurations
                    ),
                ],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )

    @staticmethod
    def __snake_to_camel(string: str) -> str:
        generator: Iterator[str] = map(str.title, string.split("_"))

        if (first_item := next(generator, None)) is not None:
            return "".join([first_item.lower()] + list(generator))
        else:
            return string

    @staticmethod
    def __config_to_cli_params(
        config: RustOpenApiGeneratorConfiguration,
    ) -> str:
        return "--additional-properties=" + ",".join(
            [
                f"{RustOpenApiGenerator.__snake_to_camel(key)}={value}"
                for (key, value) in config.items()
            ]
        )


def generate_crate(
    spec: dict[Any, Any], generator_path: str, package_name: str, path: str
) -> None:
    # Create the directory if it doesn't already exist.
    if not os.path.exists(path):
        os.makedirs(path)

    # The shared generator configurations
    generator: RustOpenApiGenerator = (
        RustOpenApiGenerator()
        .package_version("1.0.0")
        .best_fit_int(True)
        .prefer_unsigned_int(True)
        .avoid_boxed_models(True)
    )

    # Generate a sync client
    sync_client_crate_name: str = f"{package_name}-sync"
    (
        copy.deepcopy(generator)
        .package_name(sync_client_crate_name)
        .support_async(False)
    ).generate(spec, generator_path, os.path.join(path, "sync-client"), False)

    # Generate an async client
    async_client_crate_name: str = f"{package_name}-async"
    (
        copy.deepcopy(generator)
        .package_name(async_client_crate_name)
        .support_async(True)
    ).generate(spec, generator_path, os.path.join(path, "async-client"), False)

    # Generate another crate with the needed feature gates
    glue_crate_crate_name: str = package_name
    os.mkdir(os.path.join(path, package_name))

    new_manifest_file_content: dict[str, Any] = {
        "features": {
            "default": ["async"],
            "async": [f"dep:{async_client_crate_name}"],
            "sync": [f"dep:{sync_client_crate_name}"],
        },
        "dependencies": {
            sync_client_crate_name: {
                "path": f"../sync-client",
                "optional": True,
            },
            async_client_crate_name: {
                "path": f"../async-client",
                "optional": True,
            },
        },
    }
    with open(os.path.join(path, "sync-client", "Cargo.toml"), "r") as file:
        manifest_file_content: dict[str, Any] = toml.load(file)
        new_manifest_file_content["package"] = manifest_file_content["package"]
        new_manifest_file_content["package"]["name"] = glue_crate_crate_name

    with open(os.path.join(path, package_name, "Cargo.toml"), "w") as file:
        toml.dump(
            {k: v for k, v in reversed(new_manifest_file_content.items())}, file
        )

    os.mkdir(os.path.join(path, package_name, "src"))
    with open(os.path.join(path, package_name, "src", "lib.rs"), "w") as file:
        sync_client_crate_name_underscores = sync_client_crate_name.replace('-', '_')
        async_client_crate_name_underscores = async_client_crate_name.replace('-', '_')
        file.write(
            '#[cfg(all(feature = "async", feature = "sync"))]\ncompile_error!("features `sync` and `async` are mutually exclusive");\n\n'
        )
        file.write(
            f'#[cfg(feature = "sync")]\npub use {sync_client_crate_name_underscores}::*;\n'
        )
        file.write(
            f'#[cfg(feature = "async")]\npub use {async_client_crate_name_underscores}::*;\n'
        )

    pass


class Cli(
    Command,
    description="A script for generating the gateway and core API clients for Rust",
):
    node_branch: Option[Any, str] = Option(
        "-n",
        default="main",
        help="The branch of the node to get the spec from.",
    )
    gateway_branch: Option[Any, str] = Option(
        "-g",
        default="main",
        help="The branch of the gateway to get the spec from.",
    )

    def main(self):
        # Creating a temporary directory where all of the files will be saved.
        with TemporaryDirectory() as temporary_directory:
            # Downloading version 7.6.0 of the generator CLI to the temporary
            # directory.
            generator_path: str = download_openapi_generator(
                temporary_directory, "7.6.0"
            )

            # Download the OpenAPI spec of the Gateway and Core APIs and load them
            # from file.
            gateway_api_spec: dict[Any, Any]
            core_api_spec: dict[Any, Any]
            gateway_api_spec, core_api_spec = list(
                map(
                    lambda path: fix_discriminated_unions_in_spec(
                        yaml.safe_load(open(path))
                    ),
                    map(
                        lambda value: value[0](
                            temporary_directory, cast(str, value[1])
                        ),
                        [
                            (download_gateway_api_spec, self.gateway_branch),
                            (download_core_api_spec, self.node_branch),
                        ],
                    ),
                )
            )

            # If clients already exist delete them.
            generated_clients_path: str = "./generated-clients"
            if os.path.exists(generated_clients_path):
                shutil.rmtree(generated_clients_path)

            # Generate the clients.
            generate_crate(
                gateway_api_spec,
                generator_path,
                "gateway-api-client",
                os.path.join(generated_clients_path, "gateway-api-client"),
            )
            generate_crate(
                core_api_spec,
                generator_path,
                "core-api-client",
                os.path.join(generated_clients_path, "core-api-client"),
            )


if __name__ == "__main__":
    main()

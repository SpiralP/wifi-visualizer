import React from "react";
import { AddressOptions } from "./AddressNetwork";
import { IToaster, HTMLTable } from "@blueprintjs/core";

function sortAddresses(addresses: Array<[string, AddressOptions]>) {
  return addresses.sort(([id1, address1], [id2, address2]) => {
    const a = address1.signal;
    const b = address2.signal;

    if (a && !b) return -1;
    if (!a && b) return 1;

    if (a && b) {
      if (a > b) return -1;
      if (a < b) return 1;
    }

    if (id1 > id2) return 1;
    if (id1 < id2) return -1;

    return 0;
  });
}

function AccessPointsTable({
  addresses,
  onAddressHover,
}: {
  addresses: AddressOptions;
  onAddressHover: (id: string, hovered: boolean) => void;
}) {
  return (
    <HTMLTable bordered={true} interactive={true} striped={true} small={true}>
      <thead>
        <tr>
          <th>Signal</th>
          <th>Mac</th>
          <th>SSID</th>
          <th>Channel</th>
        </tr>
      </thead>
      <tbody>
        {sortAddresses(
          Object.entries(addresses).filter(
            ([_, address]) => address.accessPointInfo
          )
        ).map(([id, address]: [string, AddressOptions]) => {
          const { signal } = address;
          const { ssid, channel } = address.accessPointInfo!;

          return (
            <tr
              key={id}
              onMouseEnter={() => {
                // console.log("mouse enter");
                onAddressHover(id, true);
              }}
              onMouseLeave={() => {
                // console.log("mouse leave");
                setTimeout(() => {
                  onAddressHover(id, false);
                }, 1);
              }}
            >
              <td>{signal}</td>
              <td>{id}</td>
              <td>{ssid}</td>
              <td>{channel}</td>
            </tr>
          );
        })}
      </tbody>
    </HTMLTable>
  );
}

function StationsTable({
  addresses,
  onAddressHover,
}: {
  addresses: AddressOptions;
  onAddressHover: (id: string, hovered: boolean) => void;
}) {
  return (
    <HTMLTable bordered={true} interactive={true} striped={true} small={true}>
      <thead>
        <tr>
          <th>Signal</th>
          <th>Mac</th>
        </tr>
      </thead>
      <tbody>
        {sortAddresses(
          Object.entries(addresses).filter(
            ([_, address]) => !address.accessPointInfo
          )
        ).map(([id, address]: [string, AddressOptions]) => {
          const { signal } = address;

          return (
            <tr
              key={id}
              onMouseEnter={() => {
                onAddressHover(id, true);
              }}
              onMouseLeave={() => {
                setTimeout(() => {
                  onAddressHover(id, false);
                }, 1);
              }}
            >
              <td>{signal}</td>
              <td>{id}</td>
            </tr>
          );
        })}
      </tbody>
    </HTMLTable>
  );
}

interface AddressListProps {
  addresses: { [id: string]: AddressOptions };
  toaster: IToaster;

  onAddressHover: (id: string, hovered: boolean) => void;
}

export default class AddressList extends React.PureComponent<
  AddressListProps,
  {}
> {
  render() {
    const { addresses, onAddressHover } = this.props;

    return (
      <div>
        <AccessPointsTable
          addresses={addresses}
          onAddressHover={onAddressHover}
        />
        <br />
        <StationsTable addresses={addresses} onAddressHover={onAddressHover} />
      </div>
    );
  }
}

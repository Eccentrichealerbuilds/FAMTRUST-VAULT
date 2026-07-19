// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

contract FamilyTrust {
    address immutable owner;

    constructor(uint256 _unlocktime) {
        owner = msg.sender;
        unlockTimeByOwnerInseconds = _unlocktime;
    }

    struct Beneficiary {
        uint256 index;
        string name;
        bool isBeneficiary;
        uint256 share;
        bool isUnlocked;
    }

    uint256 private requireToUnlock;
    uint256 BeneficiariesUnlocked;
    uint256 unlockTimeByOwnerInseconds;
    uint256 finalUnlock = block.timestamp + unlockTimeByOwnerInseconds;
    string[] private namesOfBeneficiaries;
    string[] private namesOfBeneficiariesUnlocked;
    bool hasMadeFirstTransfer = false;

    mapping(address Beneficiaries => Beneficiary) ListOfBeneficiary;
    address[] private BeneficiariesAddresses;

    modifier onlyOwner() {
        require(
            msg.sender == owner,
            "UnAuthorized Action, You don't own the will, LOOSER"
        );
        _;
    }

    receive() external payable {}

    fallback() external payable {}

    function addBeneficiary(
        address newBeneficiary,
        uint256 share,
        string calldata _name
    ) external onlyOwner {
        require(
            ListOfBeneficiary[newBeneficiary].isBeneficiary == false,
            "Beneficiary already added"
        );
        require(share > 0, "Share must be greater than zero");
        require(newBeneficiary != address(0), "Address must be valid");

        ListOfBeneficiary[newBeneficiary] = Beneficiary({
            name: _name,
            share: share,
            index: namesOfBeneficiaries.length,
            isBeneficiary: true,
            isUnlocked: false
        });

        namesOfBeneficiaries.push(_name);
        BeneficiariesAddresses.push(newBeneficiary);
    }

    function modifyBeneficiaryShare(
        uint256 newShare,
        address _beneficiary
    ) public onlyOwner {
        require(
            ListOfBeneficiary[_beneficiary].isBeneficiary,
            "Not A Beneficiary"
        );
        ListOfBeneficiary[_beneficiary].share = newShare;
    }

    function unAddBeneficiary(address toUnAdd) external onlyOwner {
        if (ListOfBeneficiary[toUnAdd].isUnlocked == true) {
            BeneficiariesUnlocked -= 1;
        }
        for (uint256 i = 0; i < BeneficiariesAddresses.length; i++) {
            if (BeneficiariesAddresses[i] == toUnAdd) {
                delete BeneficiariesAddresses[i];
                break;
            }
        }
        delete namesOfBeneficiaries[ListOfBeneficiary[toUnAdd].index];
        delete ListOfBeneficiary[toUnAdd];
    }

    function unlock() external {
        require(
            ListOfBeneficiary[msg.sender].isBeneficiary == true,
            "You are not a Beneficiary, nothing for you"
        );
        require(
            ListOfBeneficiary[msg.sender].isUnlocked == false,
            "You unlocked already"
        );
        require(unlockTimeByOwnerInseconds > 0, "Unlock time not set");
        BeneficiariesUnlocked += 1;
        namesOfBeneficiariesUnlocked.push(ListOfBeneficiary[msg.sender].name);
        ListOfBeneficiary[msg.sender].isUnlocked = true;

        if (BeneficiariesUnlocked >= requireToUnlock) {
            finalUnlock = block.timestamp + unlockTimeByOwnerInseconds;
        }
    }

    function resetUnlocked() external onlyOwner {
        require(
            hasMadeFirstTransfer == false,
            "You can't reset anymore, One or More beneficiaries has made a withdrawal"
        );
        BeneficiariesUnlocked = 0;
        namesOfBeneficiariesUnlocked = new string[](0);
        finalUnlock = block.timestamp + unlockTimeByOwnerInseconds;

        for (uint256 i = 0; i < BeneficiariesAddresses.length; i++) {
            ListOfBeneficiary[BeneficiariesAddresses[i]].isUnlocked = false;
        }
    }

    function requireToUnlockAll(uint256 _required) external onlyOwner {
        requireToUnlock = _required;
    }

    function setUnlockTimeByOwner(uint256 _seconds) external onlyOwner {
        unlockTimeByOwnerInseconds = _seconds;
        finalUnlock = block.timestamp + _seconds;
    }

    function transfer() external {
        require(
            ListOfBeneficiary[msg.sender].isBeneficiary == true,
            "You are not a beneficiary sneeky, go work for your money"
        );
        require(
            ListOfBeneficiary[msg.sender].isUnlocked,
            "You have not unlocked yet"
        );
        require(block.timestamp > finalUnlock, "It's not yet time to unlock");
        if (address(this).balance < ListOfBeneficiary[msg.sender].share) {
            (bool _success, ) = payable(msg.sender).call{
                value: address(this).balance
            }("");
            require(_success, "Failed to send remaining balance");
        } else {
            (bool success, ) = payable(msg.sender).call{
                value: ListOfBeneficiary[msg.sender].share
            }("");
            require(success, "Failed to send your share");
        }
        delete ListOfBeneficiary[msg.sender];
        delete namesOfBeneficiaries[ListOfBeneficiary[msg.sender].index];
        if (hasMadeFirstTransfer == false) {
            hasMadeFirstTransfer = true;
        }
    }

    function withdraw(uint256 amount) external onlyOwner {
        (bool success, ) = payable(owner).call{value: amount}("");
        require(success, "Failed to withdraw");
    }

    function beneficiacyAllowance(
        address bene
    ) public view returns (uint256 share) {
        return ListOfBeneficiary[bene].share;
    }

    function balancOf() public view onlyOwner returns (uint256) {
        return address(this).balance;
    }

    function unlockedBenList()
        public
        view
        returns (string[] memory _unlockedBenList)
    {
        return namesOfBeneficiariesUnlocked;
    }

    function benList() public view returns (string[] memory _benList) {
        return namesOfBeneficiaries;
    }

    function _requireToUnlock() external view returns (uint256) {
        return requireToUnlock;
    }

    function unlockTime() public view onlyOwner returns (uint256) {
        return finalUnlock;
    }

    function publicUnlockTime() public view returns (uint256) {
        require(
            BeneficiariesUnlocked >= requireToUnlock,
            "Can't display unlock time until the number of unlocked beneficiary is met"
        );
        return finalUnlock;
    }
}

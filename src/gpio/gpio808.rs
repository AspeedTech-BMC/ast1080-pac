#[doc = "Register `GPIO808` reader"]
pub type R = crate::R<Gpio808Spec>;
#[doc = "Register `GPIO808` writer"]
pub type W = crate::W<Gpio808Spec>;
#[doc = "Field `WrPrivilegeOfMaster` reader - Write Privilege of Master"]
pub type WrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `WrPrivilegeOfMaster` writer - Write Privilege of Master"]
pub type WrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ReadPrivilegeOfMaster` reader - Read Privilege of Master"]
pub type ReadPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `ReadPrivilegeOfMaster` writer - Read Privilege of Master"]
pub type ReadPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `WrPrivilegeWriteProt` reader - Write Privilege Write Protection"]
pub type WrPrivilegeWriteProtR = crate::BitReader;
#[doc = "Field `WrPrivilegeWriteProt` writer - Write Privilege Write Protection"]
pub type WrPrivilegeWriteProtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ReadPrivilegeWrProt` reader - Read Privilege Write Protection"]
pub type ReadPrivilegeWrProtR = crate::BitReader;
#[doc = "Field `ReadPrivilegeWrProt` writer - Read Privilege Write Protection"]
pub type ReadPrivilegeWrProtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Write Privilege of Master"]
    #[inline(always)]
    pub fn wr_privilege_of_master(&self) -> WrPrivilegeOfMasterR {
        WrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Read Privilege of Master"]
    #[inline(always)]
    pub fn read_privilege_of_master(&self) -> ReadPrivilegeOfMasterR {
        ReadPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Write Privilege Write Protection"]
    #[inline(always)]
    pub fn wr_privilege_write_prot(&self) -> WrPrivilegeWriteProtR {
        WrPrivilegeWriteProtR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Read Privilege Write Protection"]
    #[inline(always)]
    pub fn read_privilege_wr_prot(&self) -> ReadPrivilegeWrProtR {
        ReadPrivilegeWrProtR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write Privilege of Master"]
    #[inline(always)]
    pub fn wr_privilege_of_master(&mut self) -> WrPrivilegeOfMasterW<Gpio808Spec> {
        WrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Read Privilege of Master"]
    #[inline(always)]
    pub fn read_privilege_of_master(&mut self) -> ReadPrivilegeOfMasterW<Gpio808Spec> {
        ReadPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bit 24 - Write Privilege Write Protection"]
    #[inline(always)]
    pub fn wr_privilege_write_prot(&mut self) -> WrPrivilegeWriteProtW<Gpio808Spec> {
        WrPrivilegeWriteProtW::new(self, 24)
    }
    #[doc = "Bit 25 - Read Privilege Write Protection"]
    #[inline(always)]
    pub fn read_privilege_wr_prot(&mut self) -> ReadPrivilegeWrProtW<Gpio808Spec> {
        ReadPrivilegeWrProtW::new(self, 25)
    }
}
#[doc = "GPIO Global Privilege Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio808::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio808::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio808Spec;
impl crate::RegisterSpec for Gpio808Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio808::R`](R) reader structure"]
impl crate::Readable for Gpio808Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio808::W`](W) writer structure"]
impl crate::Writable for Gpio808Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO808 to value 0xffff"]
impl crate::Resettable for Gpio808Spec {
    const RESET_VALUE: u32 = 0xffff;
}

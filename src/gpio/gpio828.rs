#[doc = "Register `GPIO828` reader"]
pub type R = crate::R<Gpio828Spec>;
#[doc = "Register `GPIO828` writer"]
pub type W = crate::W<Gpio828Spec>;
#[doc = "Field `GPIO024WrPrivilegeOfMaster` reader - GPIO024 Write Privilege of Master"]
pub type Gpio024wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO024WrPrivilegeOfMaster` writer - GPIO024 Write Privilege of Master"]
pub type Gpio024wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO025WrPrivilegeOfMaster` reader - GPIO025 Write Privilege of Master"]
pub type Gpio025wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO025WrPrivilegeOfMaster` writer - GPIO025 Write Privilege of Master"]
pub type Gpio025wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO026WrPrivilegeOfMaster` reader - GPIO026 Write Privilege of Master"]
pub type Gpio026wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO026WrPrivilegeOfMaster` writer - GPIO026 Write Privilege of Master"]
pub type Gpio026wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO027WrPrivilegeOfMaster` reader - GPIO027 Write Privilege of Master"]
pub type Gpio027wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO027WrPrivilegeOfMaster` writer - GPIO027 Write Privilege of Master"]
pub type Gpio027wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO024 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio024wr_privilege_of_master(&self) -> Gpio024wrPrivilegeOfMasterR {
        Gpio024wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO025 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio025wr_privilege_of_master(&self) -> Gpio025wrPrivilegeOfMasterR {
        Gpio025wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO026 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio026wr_privilege_of_master(&self) -> Gpio026wrPrivilegeOfMasterR {
        Gpio026wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO027 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio027wr_privilege_of_master(&self) -> Gpio027wrPrivilegeOfMasterR {
        Gpio027wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO024 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio024wr_privilege_of_master(&mut self) -> Gpio024wrPrivilegeOfMasterW<Gpio828Spec> {
        Gpio024wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO025 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio025wr_privilege_of_master(&mut self) -> Gpio025wrPrivilegeOfMasterW<Gpio828Spec> {
        Gpio025wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO026 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio026wr_privilege_of_master(&mut self) -> Gpio026wrPrivilegeOfMasterW<Gpio828Spec> {
        Gpio026wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO027 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio027wr_privilege_of_master(&mut self) -> Gpio027wrPrivilegeOfMasterW<Gpio828Spec> {
        Gpio027wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio828::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio828::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio828Spec;
impl crate::RegisterSpec for Gpio828Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio828::R`](R) reader structure"]
impl crate::Readable for Gpio828Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio828::W`](W) writer structure"]
impl crate::Writable for Gpio828Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO828 to value 0xffff_ffff"]
impl crate::Resettable for Gpio828Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

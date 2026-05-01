#[doc = "Register `GPIO8C0` reader"]
pub type R = crate::R<Gpio8c0Spec>;
#[doc = "Register `GPIO8C0` writer"]
pub type W = crate::W<Gpio8c0Spec>;
#[doc = "Field `GPIO176WrPrivilegeOfMaster` reader - GPIO176 Write Privilege of Master"]
pub type Gpio176wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO176WrPrivilegeOfMaster` writer - GPIO176 Write Privilege of Master"]
pub type Gpio176wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO177WrPrivilegeOfMaster` reader - GPIO177 Write Privilege of Master"]
pub type Gpio177wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO177WrPrivilegeOfMaster` writer - GPIO177 Write Privilege of Master"]
pub type Gpio177wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO178WrPrivilegeOfMaster` reader - GPIO178 Write Privilege of Master"]
pub type Gpio178wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO178WrPrivilegeOfMaster` writer - GPIO178 Write Privilege of Master"]
pub type Gpio178wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO179WrPrivilegeOfMaster` reader - GPIO179 Write Privilege of Master"]
pub type Gpio179wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO179WrPrivilegeOfMaster` writer - GPIO179 Write Privilege of Master"]
pub type Gpio179wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO176 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio176wr_privilege_of_master(&self) -> Gpio176wrPrivilegeOfMasterR {
        Gpio176wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO177 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio177wr_privilege_of_master(&self) -> Gpio177wrPrivilegeOfMasterR {
        Gpio177wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO178 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio178wr_privilege_of_master(&self) -> Gpio178wrPrivilegeOfMasterR {
        Gpio178wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO179 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio179wr_privilege_of_master(&self) -> Gpio179wrPrivilegeOfMasterR {
        Gpio179wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO176 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio176wr_privilege_of_master(&mut self) -> Gpio176wrPrivilegeOfMasterW<Gpio8c0Spec> {
        Gpio176wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO177 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio177wr_privilege_of_master(&mut self) -> Gpio177wrPrivilegeOfMasterW<Gpio8c0Spec> {
        Gpio177wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO178 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio178wr_privilege_of_master(&mut self) -> Gpio178wrPrivilegeOfMasterW<Gpio8c0Spec> {
        Gpio178wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO179 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio179wr_privilege_of_master(&mut self) -> Gpio179wrPrivilegeOfMasterW<Gpio8c0Spec> {
        Gpio179wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#44\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio8c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio8c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio8c0Spec;
impl crate::RegisterSpec for Gpio8c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio8c0::R`](R) reader structure"]
impl crate::Readable for Gpio8c0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio8c0::W`](W) writer structure"]
impl crate::Writable for Gpio8c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO8C0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio8c0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

#[doc = "Register `GPIO9B4` reader"]
pub type R = crate::R<Gpio9b4Spec>;
#[doc = "Register `GPIO9B4` writer"]
pub type W = crate::W<Gpio9b4Spec>;
#[doc = "Field `GPIO164ReadPrivilegeOfMaster` reader - GPIO164 Read Privilege of Master"]
pub type Gpio164readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO164ReadPrivilegeOfMaster` writer - GPIO164 Read Privilege of Master"]
pub type Gpio164readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO165ReadPrivilegeOfMaster` reader - GPIO165 Read Privilege of Master"]
pub type Gpio165readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO165ReadPrivilegeOfMaster` writer - GPIO165 Read Privilege of Master"]
pub type Gpio165readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO166ReadPrivilegeOfMaster` reader - GPIO166 Read Privilege of Master"]
pub type Gpio166readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO166ReadPrivilegeOfMaster` writer - GPIO166 Read Privilege of Master"]
pub type Gpio166readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO167ReadPrivilegeOfMaster` reader - GPIO167 Read Privilege of Master"]
pub type Gpio167readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO167ReadPrivilegeOfMaster` writer - GPIO167 Read Privilege of Master"]
pub type Gpio167readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO164 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio164read_privilege_of_master(&self) -> Gpio164readPrivilegeOfMasterR {
        Gpio164readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO165 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio165read_privilege_of_master(&self) -> Gpio165readPrivilegeOfMasterR {
        Gpio165readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO166 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio166read_privilege_of_master(&self) -> Gpio166readPrivilegeOfMasterR {
        Gpio166readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO167 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio167read_privilege_of_master(&self) -> Gpio167readPrivilegeOfMasterR {
        Gpio167readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO164 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio164read_privilege_of_master(
        &mut self,
    ) -> Gpio164readPrivilegeOfMasterW<Gpio9b4Spec> {
        Gpio164readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO165 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio165read_privilege_of_master(
        &mut self,
    ) -> Gpio165readPrivilegeOfMasterW<Gpio9b4Spec> {
        Gpio165readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO166 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio166read_privilege_of_master(
        &mut self,
    ) -> Gpio166readPrivilegeOfMasterW<Gpio9b4Spec> {
        Gpio166readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO167 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio167read_privilege_of_master(
        &mut self,
    ) -> Gpio167readPrivilegeOfMasterW<Gpio9b4Spec> {
        Gpio167readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#41\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9b4Spec;
impl crate::RegisterSpec for Gpio9b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9b4::R`](R) reader structure"]
impl crate::Readable for Gpio9b4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9b4::W`](W) writer structure"]
impl crate::Writable for Gpio9b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9B4 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9b4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

#[doc = "Register `GPIO884` reader"]
pub type R = crate::R<Gpio884Spec>;
#[doc = "Register `GPIO884` writer"]
pub type W = crate::W<Gpio884Spec>;
#[doc = "Field `GPIO116WrPrivilegeOfMaster` reader - GPIO116 Write Privilege of Master"]
pub type Gpio116wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO116WrPrivilegeOfMaster` writer - GPIO116 Write Privilege of Master"]
pub type Gpio116wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO117WrPrivilegeOfMaster` reader - GPIO117 Write Privilege of Master"]
pub type Gpio117wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO117WrPrivilegeOfMaster` writer - GPIO117 Write Privilege of Master"]
pub type Gpio117wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO118WrPrivilegeOfMaster` reader - GPIO118 Write Privilege of Master"]
pub type Gpio118wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO118WrPrivilegeOfMaster` writer - GPIO118 Write Privilege of Master"]
pub type Gpio118wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO119WrPrivilegeOfMaster` reader - GPIO119 Write Privilege of Master"]
pub type Gpio119wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO119WrPrivilegeOfMaster` writer - GPIO119 Write Privilege of Master"]
pub type Gpio119wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO116 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio116wr_privilege_of_master(&self) -> Gpio116wrPrivilegeOfMasterR {
        Gpio116wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO117 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio117wr_privilege_of_master(&self) -> Gpio117wrPrivilegeOfMasterR {
        Gpio117wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO118 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio118wr_privilege_of_master(&self) -> Gpio118wrPrivilegeOfMasterR {
        Gpio118wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO119 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio119wr_privilege_of_master(&self) -> Gpio119wrPrivilegeOfMasterR {
        Gpio119wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO116 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio116wr_privilege_of_master(&mut self) -> Gpio116wrPrivilegeOfMasterW<Gpio884Spec> {
        Gpio116wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO117 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio117wr_privilege_of_master(&mut self) -> Gpio117wrPrivilegeOfMasterW<Gpio884Spec> {
        Gpio117wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO118 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio118wr_privilege_of_master(&mut self) -> Gpio118wrPrivilegeOfMasterW<Gpio884Spec> {
        Gpio118wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO119 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio119wr_privilege_of_master(&mut self) -> Gpio119wrPrivilegeOfMasterW<Gpio884Spec> {
        Gpio119wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio884::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio884::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio884Spec;
impl crate::RegisterSpec for Gpio884Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio884::R`](R) reader structure"]
impl crate::Readable for Gpio884Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio884::W`](W) writer structure"]
impl crate::Writable for Gpio884Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO884 to value 0xffff_ffff"]
impl crate::Resettable for Gpio884Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

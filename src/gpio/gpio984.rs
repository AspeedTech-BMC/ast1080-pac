#[doc = "Register `GPIO984` reader"]
pub type R = crate::R<Gpio984Spec>;
#[doc = "Register `GPIO984` writer"]
pub type W = crate::W<Gpio984Spec>;
#[doc = "Field `GPIO116ReadPrivilegeOfMaster` reader - GPIO116 Read Privilege of Master"]
pub type Gpio116readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO116ReadPrivilegeOfMaster` writer - GPIO116 Read Privilege of Master"]
pub type Gpio116readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO117ReadPrivilegeOfMaster` reader - GPIO117 Read Privilege of Master"]
pub type Gpio117readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO117ReadPrivilegeOfMaster` writer - GPIO117 Read Privilege of Master"]
pub type Gpio117readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO118ReadPrivilegeOfMaster` reader - GPIO118 Read Privilege of Master"]
pub type Gpio118readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO118ReadPrivilegeOfMaster` writer - GPIO118 Read Privilege of Master"]
pub type Gpio118readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO119ReadPrivilegeOfMaster` reader - GPIO119 Read Privilege of Master"]
pub type Gpio119readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO119ReadPrivilegeOfMaster` writer - GPIO119 Read Privilege of Master"]
pub type Gpio119readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO116 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio116read_privilege_of_master(&self) -> Gpio116readPrivilegeOfMasterR {
        Gpio116readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO117 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio117read_privilege_of_master(&self) -> Gpio117readPrivilegeOfMasterR {
        Gpio117readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO118 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio118read_privilege_of_master(&self) -> Gpio118readPrivilegeOfMasterR {
        Gpio118readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO119 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio119read_privilege_of_master(&self) -> Gpio119readPrivilegeOfMasterR {
        Gpio119readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO116 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio116read_privilege_of_master(
        &mut self,
    ) -> Gpio116readPrivilegeOfMasterW<Gpio984Spec> {
        Gpio116readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO117 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio117read_privilege_of_master(
        &mut self,
    ) -> Gpio117readPrivilegeOfMasterW<Gpio984Spec> {
        Gpio117readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO118 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio118read_privilege_of_master(
        &mut self,
    ) -> Gpio118readPrivilegeOfMasterW<Gpio984Spec> {
        Gpio118readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO119 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio119read_privilege_of_master(
        &mut self,
    ) -> Gpio119readPrivilegeOfMasterW<Gpio984Spec> {
        Gpio119readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#29\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio984::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio984::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio984Spec;
impl crate::RegisterSpec for Gpio984Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio984::R`](R) reader structure"]
impl crate::Readable for Gpio984Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio984::W`](W) writer structure"]
impl crate::Writable for Gpio984Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO984 to value 0xffff_ffff"]
impl crate::Resettable for Gpio984Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

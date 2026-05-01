#[doc = "Register `GPIO9C4` reader"]
pub type R = crate::R<Gpio9c4Spec>;
#[doc = "Register `GPIO9C4` writer"]
pub type W = crate::W<Gpio9c4Spec>;
#[doc = "Field `GPIO180ReadPrivilegeOfMaster` reader - GPIO180 Read Privilege of Master"]
pub type Gpio180readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO180ReadPrivilegeOfMaster` writer - GPIO180 Read Privilege of Master"]
pub type Gpio180readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO181ReadPrivilegeOfMaster` reader - GPIO181 Read Privilege of Master"]
pub type Gpio181readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO181ReadPrivilegeOfMaster` writer - GPIO181 Read Privilege of Master"]
pub type Gpio181readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO182ReadPrivilegeOfMaster` reader - GPIO182 Read Privilege of Master"]
pub type Gpio182readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO182ReadPrivilegeOfMaster` writer - GPIO182 Read Privilege of Master"]
pub type Gpio182readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO183ReadPrivilegeOfMaster` reader - GPIO183 Read Privilege of Master"]
pub type Gpio183readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO183ReadPrivilegeOfMaster` writer - GPIO183 Read Privilege of Master"]
pub type Gpio183readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO180 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio180read_privilege_of_master(&self) -> Gpio180readPrivilegeOfMasterR {
        Gpio180readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO181 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio181read_privilege_of_master(&self) -> Gpio181readPrivilegeOfMasterR {
        Gpio181readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO182 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio182read_privilege_of_master(&self) -> Gpio182readPrivilegeOfMasterR {
        Gpio182readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO183 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio183read_privilege_of_master(&self) -> Gpio183readPrivilegeOfMasterR {
        Gpio183readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO180 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio180read_privilege_of_master(
        &mut self,
    ) -> Gpio180readPrivilegeOfMasterW<Gpio9c4Spec> {
        Gpio180readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO181 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio181read_privilege_of_master(
        &mut self,
    ) -> Gpio181readPrivilegeOfMasterW<Gpio9c4Spec> {
        Gpio181readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO182 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio182read_privilege_of_master(
        &mut self,
    ) -> Gpio182readPrivilegeOfMasterW<Gpio9c4Spec> {
        Gpio182readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO183 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio183read_privilege_of_master(
        &mut self,
    ) -> Gpio183readPrivilegeOfMasterW<Gpio9c4Spec> {
        Gpio183readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#45\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9c4Spec;
impl crate::RegisterSpec for Gpio9c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9c4::R`](R) reader structure"]
impl crate::Readable for Gpio9c4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9c4::W`](W) writer structure"]
impl crate::Writable for Gpio9c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9C4 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9c4Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

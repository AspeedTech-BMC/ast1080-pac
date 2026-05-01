#[doc = "Register `GPIO9C8` reader"]
pub type R = crate::R<Gpio9c8Spec>;
#[doc = "Register `GPIO9C8` writer"]
pub type W = crate::W<Gpio9c8Spec>;
#[doc = "Field `GPIO184ReadPrivilegeOfMaster` reader - GPIO184 Read Privilege of Master"]
pub type Gpio184readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO184ReadPrivilegeOfMaster` writer - GPIO184 Read Privilege of Master"]
pub type Gpio184readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO185ReadPrivilegeOfMaster` reader - GPIO185 Read Privilege of Master"]
pub type Gpio185readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO185ReadPrivilegeOfMaster` writer - GPIO185 Read Privilege of Master"]
pub type Gpio185readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO186ReadPrivilegeOfMaster` reader - GPIO186 Read Privilege of Master"]
pub type Gpio186readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO186ReadPrivilegeOfMaster` writer - GPIO186 Read Privilege of Master"]
pub type Gpio186readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO187ReadPrivilegeOfMaster` reader - GPIO187 Read Privilege of Master"]
pub type Gpio187readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO187ReadPrivilegeOfMaster` writer - GPIO187 Read Privilege of Master"]
pub type Gpio187readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO184 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio184read_privilege_of_master(&self) -> Gpio184readPrivilegeOfMasterR {
        Gpio184readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO185 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio185read_privilege_of_master(&self) -> Gpio185readPrivilegeOfMasterR {
        Gpio185readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO186 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio186read_privilege_of_master(&self) -> Gpio186readPrivilegeOfMasterR {
        Gpio186readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO187 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio187read_privilege_of_master(&self) -> Gpio187readPrivilegeOfMasterR {
        Gpio187readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO184 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio184read_privilege_of_master(
        &mut self,
    ) -> Gpio184readPrivilegeOfMasterW<Gpio9c8Spec> {
        Gpio184readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO185 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio185read_privilege_of_master(
        &mut self,
    ) -> Gpio185readPrivilegeOfMasterW<Gpio9c8Spec> {
        Gpio185readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO186 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio186read_privilege_of_master(
        &mut self,
    ) -> Gpio186readPrivilegeOfMasterW<Gpio9c8Spec> {
        Gpio186readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO187 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio187read_privilege_of_master(
        &mut self,
    ) -> Gpio187readPrivilegeOfMasterW<Gpio9c8Spec> {
        Gpio187readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#46\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9c8Spec;
impl crate::RegisterSpec for Gpio9c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9c8::R`](R) reader structure"]
impl crate::Readable for Gpio9c8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9c8::W`](W) writer structure"]
impl crate::Writable for Gpio9c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9C8 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9c8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

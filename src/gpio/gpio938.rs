#[doc = "Register `GPIO938` reader"]
pub type R = crate::R<Gpio938Spec>;
#[doc = "Register `GPIO938` writer"]
pub type W = crate::W<Gpio938Spec>;
#[doc = "Field `GPIO040ReadPrivilegeOfMaster` reader - GPIO040 Read Privilege of Master"]
pub type Gpio040readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO040ReadPrivilegeOfMaster` writer - GPIO040 Read Privilege of Master"]
pub type Gpio040readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO041ReadPrivilegeOfMaster` reader - GPIO041 Read Privilege of Master"]
pub type Gpio041readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO041ReadPrivilegeOfMaster` writer - GPIO041 Read Privilege of Master"]
pub type Gpio041readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO042ReadPrivilegeOfMaster` reader - GPIO042 Read Privilege of Master"]
pub type Gpio042readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO042ReadPrivilegeOfMaster` writer - GPIO042 Read Privilege of Master"]
pub type Gpio042readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO043ReadPrivilegeOfMaster` reader - GPIO043 Read Privilege of Master"]
pub type Gpio043readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO043ReadPrivilegeOfMaster` writer - GPIO043 Read Privilege of Master"]
pub type Gpio043readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO040 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio040read_privilege_of_master(&self) -> Gpio040readPrivilegeOfMasterR {
        Gpio040readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO041 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio041read_privilege_of_master(&self) -> Gpio041readPrivilegeOfMasterR {
        Gpio041readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO042 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio042read_privilege_of_master(&self) -> Gpio042readPrivilegeOfMasterR {
        Gpio042readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO043 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio043read_privilege_of_master(&self) -> Gpio043readPrivilegeOfMasterR {
        Gpio043readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO040 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio040read_privilege_of_master(
        &mut self,
    ) -> Gpio040readPrivilegeOfMasterW<Gpio938Spec> {
        Gpio040readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO041 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio041read_privilege_of_master(
        &mut self,
    ) -> Gpio041readPrivilegeOfMasterW<Gpio938Spec> {
        Gpio041readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO042 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio042read_privilege_of_master(
        &mut self,
    ) -> Gpio042readPrivilegeOfMasterW<Gpio938Spec> {
        Gpio042readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO043 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio043read_privilege_of_master(
        &mut self,
    ) -> Gpio043readPrivilegeOfMasterW<Gpio938Spec> {
        Gpio043readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio938::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio938::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio938Spec;
impl crate::RegisterSpec for Gpio938Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio938::R`](R) reader structure"]
impl crate::Readable for Gpio938Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio938::W`](W) writer structure"]
impl crate::Writable for Gpio938Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO938 to value 0xffff_ffff"]
impl crate::Resettable for Gpio938Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

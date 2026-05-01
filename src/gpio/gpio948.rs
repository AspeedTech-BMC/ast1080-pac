#[doc = "Register `GPIO948` reader"]
pub type R = crate::R<Gpio948Spec>;
#[doc = "Register `GPIO948` writer"]
pub type W = crate::W<Gpio948Spec>;
#[doc = "Field `GPIO056ReadPrivilegeOfMaster` reader - GPIO056 Read Privilege of Master"]
pub type Gpio056readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO056ReadPrivilegeOfMaster` writer - GPIO056 Read Privilege of Master"]
pub type Gpio056readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO057ReadPrivilegeOfMaster` reader - GPIO057 Read Privilege of Master"]
pub type Gpio057readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO057ReadPrivilegeOfMaster` writer - GPIO057 Read Privilege of Master"]
pub type Gpio057readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO058ReadPrivilegeOfMaster` reader - GPIO058 Read Privilege of Master"]
pub type Gpio058readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO058ReadPrivilegeOfMaster` writer - GPIO058 Read Privilege of Master"]
pub type Gpio058readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO059ReadPrivilegeOfMaster` reader - GPIO059 Read Privilege of Master"]
pub type Gpio059readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO059ReadPrivilegeOfMaster` writer - GPIO059 Read Privilege of Master"]
pub type Gpio059readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO056 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio056read_privilege_of_master(&self) -> Gpio056readPrivilegeOfMasterR {
        Gpio056readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO057 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio057read_privilege_of_master(&self) -> Gpio057readPrivilegeOfMasterR {
        Gpio057readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO058 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio058read_privilege_of_master(&self) -> Gpio058readPrivilegeOfMasterR {
        Gpio058readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO059 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio059read_privilege_of_master(&self) -> Gpio059readPrivilegeOfMasterR {
        Gpio059readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO056 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio056read_privilege_of_master(
        &mut self,
    ) -> Gpio056readPrivilegeOfMasterW<Gpio948Spec> {
        Gpio056readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO057 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio057read_privilege_of_master(
        &mut self,
    ) -> Gpio057readPrivilegeOfMasterW<Gpio948Spec> {
        Gpio057readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO058 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio058read_privilege_of_master(
        &mut self,
    ) -> Gpio058readPrivilegeOfMasterW<Gpio948Spec> {
        Gpio058readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO059 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio059read_privilege_of_master(
        &mut self,
    ) -> Gpio059readPrivilegeOfMasterW<Gpio948Spec> {
        Gpio059readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#14\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio948::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio948::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio948Spec;
impl crate::RegisterSpec for Gpio948Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio948::R`](R) reader structure"]
impl crate::Readable for Gpio948Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio948::W`](W) writer structure"]
impl crate::Writable for Gpio948Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO948 to value 0xffff_ffff"]
impl crate::Resettable for Gpio948Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

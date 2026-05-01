#[doc = "Register `GPIO954` reader"]
pub type R = crate::R<Gpio954Spec>;
#[doc = "Register `GPIO954` writer"]
pub type W = crate::W<Gpio954Spec>;
#[doc = "Field `GPIO068ReadPrivilegeOfMaster` reader - GPIO068 Read Privilege of Master"]
pub type Gpio068readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO068ReadPrivilegeOfMaster` writer - GPIO068 Read Privilege of Master"]
pub type Gpio068readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO069ReadPrivilegeOfMaster` reader - GPIO069 Read Privilege of Master"]
pub type Gpio069readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO069ReadPrivilegeOfMaster` writer - GPIO069 Read Privilege of Master"]
pub type Gpio069readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO070ReadPrivilegeOfMaster` reader - GPIO070 Read Privilege of Master"]
pub type Gpio070readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO070ReadPrivilegeOfMaster` writer - GPIO070 Read Privilege of Master"]
pub type Gpio070readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO071ReadPrivilegeOfMaster` reader - GPIO071 Read Privilege of Master"]
pub type Gpio071readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO071ReadPrivilegeOfMaster` writer - GPIO071 Read Privilege of Master"]
pub type Gpio071readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO068 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio068read_privilege_of_master(&self) -> Gpio068readPrivilegeOfMasterR {
        Gpio068readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO069 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio069read_privilege_of_master(&self) -> Gpio069readPrivilegeOfMasterR {
        Gpio069readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO070 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio070read_privilege_of_master(&self) -> Gpio070readPrivilegeOfMasterR {
        Gpio070readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO071 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio071read_privilege_of_master(&self) -> Gpio071readPrivilegeOfMasterR {
        Gpio071readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO068 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio068read_privilege_of_master(
        &mut self,
    ) -> Gpio068readPrivilegeOfMasterW<Gpio954Spec> {
        Gpio068readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO069 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio069read_privilege_of_master(
        &mut self,
    ) -> Gpio069readPrivilegeOfMasterW<Gpio954Spec> {
        Gpio069readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO070 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio070read_privilege_of_master(
        &mut self,
    ) -> Gpio070readPrivilegeOfMasterW<Gpio954Spec> {
        Gpio070readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO071 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio071read_privilege_of_master(
        &mut self,
    ) -> Gpio071readPrivilegeOfMasterW<Gpio954Spec> {
        Gpio071readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#17\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio954::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio954::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio954Spec;
impl crate::RegisterSpec for Gpio954Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio954::R`](R) reader structure"]
impl crate::Readable for Gpio954Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio954::W`](W) writer structure"]
impl crate::Writable for Gpio954Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO954 to value 0xffff_ffff"]
impl crate::Resettable for Gpio954Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

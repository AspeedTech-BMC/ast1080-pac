#[doc = "Register `GPIO910` reader"]
pub type R = crate::R<Gpio910Spec>;
#[doc = "Register `GPIO910` writer"]
pub type W = crate::W<Gpio910Spec>;
#[doc = "Field `GPIO000ReadPrivilegeOfMaster` reader - GPIO000 Read Privilege of Master"]
pub type Gpio000readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO000ReadPrivilegeOfMaster` writer - GPIO000 Read Privilege of Master"]
pub type Gpio000readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO001ReadPrivilegeOfMaster` reader - GPIO001 Read Privilege of Master"]
pub type Gpio001readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO001ReadPrivilegeOfMaster` writer - GPIO001 Read Privilege of Master"]
pub type Gpio001readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO002ReadPrivilegeOfMaster` reader - GPIO002 Read Privilege of Master"]
pub type Gpio002readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO002ReadPrivilegeOfMaster` writer - GPIO002 Read Privilege of Master"]
pub type Gpio002readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO003ReadPrivilegeOfMaster` reader - GPIO003 Read Privilege of Master"]
pub type Gpio003readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO003ReadPrivilegeOfMaster` writer - GPIO003 Read Privilege of Master"]
pub type Gpio003readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO000 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio000read_privilege_of_master(&self) -> Gpio000readPrivilegeOfMasterR {
        Gpio000readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO001 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio001read_privilege_of_master(&self) -> Gpio001readPrivilegeOfMasterR {
        Gpio001readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO002 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio002read_privilege_of_master(&self) -> Gpio002readPrivilegeOfMasterR {
        Gpio002readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO003 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio003read_privilege_of_master(&self) -> Gpio003readPrivilegeOfMasterR {
        Gpio003readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO000 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio000read_privilege_of_master(
        &mut self,
    ) -> Gpio000readPrivilegeOfMasterW<Gpio910Spec> {
        Gpio000readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO001 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio001read_privilege_of_master(
        &mut self,
    ) -> Gpio001readPrivilegeOfMasterW<Gpio910Spec> {
        Gpio001readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO002 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio002read_privilege_of_master(
        &mut self,
    ) -> Gpio002readPrivilegeOfMasterW<Gpio910Spec> {
        Gpio002readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO003 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio003read_privilege_of_master(
        &mut self,
    ) -> Gpio003readPrivilegeOfMasterW<Gpio910Spec> {
        Gpio003readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio910::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio910::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio910Spec;
impl crate::RegisterSpec for Gpio910Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio910::R`](R) reader structure"]
impl crate::Readable for Gpio910Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio910::W`](W) writer structure"]
impl crate::Writable for Gpio910Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO910 to value 0xffff_ffff"]
impl crate::Resettable for Gpio910Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

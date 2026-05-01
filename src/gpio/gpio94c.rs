#[doc = "Register `GPIO94C` reader"]
pub type R = crate::R<Gpio94cSpec>;
#[doc = "Register `GPIO94C` writer"]
pub type W = crate::W<Gpio94cSpec>;
#[doc = "Field `GPIO060ReadPrivilegeOfMaster` reader - GPIO060 Read Privilege of Master"]
pub type Gpio060readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO060ReadPrivilegeOfMaster` writer - GPIO060 Read Privilege of Master"]
pub type Gpio060readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO061ReadPrivilegeOfMaster` reader - GPIO061 Read Privilege of Master"]
pub type Gpio061readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO061ReadPrivilegeOfMaster` writer - GPIO061 Read Privilege of Master"]
pub type Gpio061readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO062ReadPrivilegeOfMaster` reader - GPIO062 Read Privilege of Master"]
pub type Gpio062readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO062ReadPrivilegeOfMaster` writer - GPIO062 Read Privilege of Master"]
pub type Gpio062readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO063ReadPrivilegeOfMaster` reader - GPIO063 Read Privilege of Master"]
pub type Gpio063readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO063ReadPrivilegeOfMaster` writer - GPIO063 Read Privilege of Master"]
pub type Gpio063readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO060 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio060read_privilege_of_master(&self) -> Gpio060readPrivilegeOfMasterR {
        Gpio060readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO061 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio061read_privilege_of_master(&self) -> Gpio061readPrivilegeOfMasterR {
        Gpio061readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO062 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio062read_privilege_of_master(&self) -> Gpio062readPrivilegeOfMasterR {
        Gpio062readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO063 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio063read_privilege_of_master(&self) -> Gpio063readPrivilegeOfMasterR {
        Gpio063readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO060 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio060read_privilege_of_master(
        &mut self,
    ) -> Gpio060readPrivilegeOfMasterW<Gpio94cSpec> {
        Gpio060readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO061 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio061read_privilege_of_master(
        &mut self,
    ) -> Gpio061readPrivilegeOfMasterW<Gpio94cSpec> {
        Gpio061readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO062 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio062read_privilege_of_master(
        &mut self,
    ) -> Gpio062readPrivilegeOfMasterW<Gpio94cSpec> {
        Gpio062readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO063 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio063read_privilege_of_master(
        &mut self,
    ) -> Gpio063readPrivilegeOfMasterW<Gpio94cSpec> {
        Gpio063readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#15\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio94c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio94c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio94cSpec;
impl crate::RegisterSpec for Gpio94cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio94c::R`](R) reader structure"]
impl crate::Readable for Gpio94cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio94c::W`](W) writer structure"]
impl crate::Writable for Gpio94cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO94C to value 0xffff_ffff"]
impl crate::Resettable for Gpio94cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
